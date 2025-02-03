use std::time::Instant;

use axum::{
    extract::State,
    http::{HeaderMap, HeaderValue, StatusCode},
    Json,
};
use models::{
    api::{application_key::ApplicationKey, request::Request, user_key::UserKey},
    source::{get_sources_for_user_key_and_application_key, Sources},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::PgPool;
use tracing::{error, warn};

use crate::{
    plan::{PlanManager, PlanResponse, PlanSuccessResponse},
    AppState,
};

const ALLOWED_PLAN_TRIALS: usize = 2;

#[derive(Deserialize, Serialize)]
pub struct UserData {
    id: i64,
    organization_id: i64,
}

#[derive(Deserialize, Serialize)]
pub struct RequestBody {
    query: String,
    user: Option<UserData>,
}

#[derive(Debug, Clone)]
pub struct AuthData {
    pub user_key: UserKey,
    pub app_key: ApplicationKey,
    pub available_sources: Sources,
}

pub fn update_request_info(
    mut request: Request,
    start: Instant,
    response_status: StatusCode,
    response_body: Option<PlanSuccessResponse>,
    pool: &PgPool,
) {
    let duration = start.elapsed();
    request.duration_ms = Some(duration.as_millis() as i32);
    request.response_status = Some(response_status.as_u16() as i32);
    request.response_body = response_body.map(|body| serde_json::to_value(body).unwrap());
    let local_pool = pool.clone();
    tokio::spawn(async move {
        let _ = request.update_in_database(&local_pool).await;
    });
}

async fn get_user_key(
    headers: &HeaderMap,
    body: Option<&RequestBody>,
    app_key: &ApplicationKey,
    pool: &PgPool,
) -> Result<UserKey, StatusCode> {
    // If we have a user key use it
    if let Some(auth_header) = headers.get("Authorization") {
        return get_user_key_from_auth_header(auth_header, pool).await;
    }
    // If we don't have a user key and the app_key is an admin key check for user
    // data in the body
    if app_key.is_admin {
        if let Some(Some(user)) = body.map(|body| &body.user) {
            return get_admin_key_for_user(user.id, user.organization_id, pool).await;
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}

async fn get_user_key_from_auth_header(
    auth_header: &HeaderValue,
    pool: &PgPool,
) -> Result<UserKey, StatusCode> {
    let header_str = auth_header.to_str().map_err(|_| StatusCode::UNAUTHORIZED)?;

    let token = header_str
        .strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let mut parts = token.split('_');
    let key_id = parts.next().ok_or(StatusCode::UNAUTHORIZED)?;
    let key = parts.next().ok_or(StatusCode::UNAUTHORIZED)?;

    UserKey::get_with_key_id_key(key_id, key, pool)
        .await
        .map_err(|e| {
            error!("getting user key: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::UNAUTHORIZED)
}

async fn get_admin_key_for_user(
    user_id: i64,
    org_id: i64,
    pool: &PgPool,
) -> Result<UserKey, StatusCode> {
    UserKey::get_admin_key(user_id, org_id, pool)
        .await
        .map_err(|e| {
            error!("getting user key: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::UNAUTHORIZED)
}

async fn get_app_key(headers: &HeaderMap, pool: &PgPool) -> Result<ApplicationKey, StatusCode> {
    let mut app_key = headers
        .get("X-API-Key")
        .and_then(|header| header.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?
        .split("_");
    let app_key_id = app_key.next().ok_or(StatusCode::UNAUTHORIZED)?;
    let app_key_key = app_key.next().ok_or(StatusCode::UNAUTHORIZED)?;

    let app_key = ApplicationKey::get_with_key_id_key(app_key_id, app_key_key, pool)
        .await
        .map_err(|e| {
            error!("getting app key: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    app_key.ok_or(StatusCode::UNAUTHORIZED)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourcesResponse {
    pub sources: Sources,
}

// TODO: @silas add logging for this
pub async fn sources(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<(StatusCode, Json<Value>), StatusCode> {
    // 1. Auth the user
    let app_key = get_app_key(&headers, &state.pool).await?;
    let user_key = get_user_key(&headers, None, &app_key, &state.pool).await?;

    // 2. Get the sources
    let available_sources =
        get_sources_for_user_key_and_application_key(&user_key, &app_key, &state.pool)
            .await
            .map_err(|e| {
                error!("getting sources: {e:?}");
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

    // 3. Update the request and return the sources
    Ok((
        StatusCode::OK,
        Json(
            serde_json::to_value(SourcesResponse {
                sources: available_sources,
            })
            .unwrap(),
        ),
    ))
}

pub async fn context(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(body): Json<RequestBody>,
) -> Result<(StatusCode, Json<Value>), StatusCode> {
    let start = Instant::now();

    // 1. Auth the user
    let app_key = get_app_key(&headers, &state.pool).await?;
    let user_key = get_user_key(&headers, Some(&body), &app_key, &state.pool).await?;

    // 2. Log the request
    let request = Request::create(
        user_key.id,
        app_key.id,
        1,
        &serde_json::to_value(&body).unwrap(),
        &state.pool,
    )
    .await
    .map_err(|e| {
        error!("creating request: {e:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // 3. Get the users sources available to the current api key and organization
    //    key combo
    let available_sources =
        get_sources_for_user_key_and_application_key(&user_key, &app_key, &state.pool)
            .await
            .map_err(|e| {
                error!("getting sources: {e:?}");
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

    // 4. Execute the query
    let auth_data = AuthData {
        user_key,
        app_key,
        available_sources,
    };

    let mut plan_manager = PlanManager::new(&body.query);
    let mut trial_count = 1;
    let plan_response = loop {
        match plan_manager.execute(&auth_data, &state.pool).await {
            Ok(resp) => {
                break resp;
            }
            Err(err) => {
                warn!(
                    "request: {} - plan error for trial: {} - {err:?}",
                    request.id, trial_count
                );

                if trial_count == ALLOWED_PLAN_TRIALS {
                    update_request_info(
                        request,
                        start,
                        StatusCode::INTERNAL_SERVER_ERROR,
                        None,
                        &state.pool,
                    );
                    return Err(StatusCode::INTERNAL_SERVER_ERROR);
                }
            }
        }
        trial_count += 1;
    };

    // 5. Respond
    match plan_response {
        PlanResponse::Success(response) => {
            update_request_info(
                request,
                start,
                StatusCode::OK,
                Some(response.clone()),
                &state.pool,
            );
            Ok((
                StatusCode::OK,
                Json(serde_json::to_value(response).unwrap()),
            ))
        }
        PlanResponse::Failure(response) => {
            warn!(
                "request: {} - plan failure: {}",
                request.id,
                serde_json::to_string(&response).unwrap()
            );
            update_request_info(
                request,
                start,
                StatusCode::UNPROCESSABLE_ENTITY,
                None,
                &state.pool,
            );
            Ok((
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(serde_json::to_value(response).unwrap()),
            ))
        }
    }
}
