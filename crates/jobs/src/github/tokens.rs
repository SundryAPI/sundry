use std::{
    collections::HashMap,
    env,
    sync::{Arc, LazyLock},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use reqwest::{Client, IntoUrl};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tokio::sync::Mutex;

const EXPIRATION_BUFFER: Duration = Duration::from_secs(300);
const TTL: Duration = Duration::from_secs(3600);

pub static MANAGER: LazyLock<Arc<GithubTokenManager>> =
    LazyLock::new(|| Arc::new(GithubTokenManager::default()));

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    iat: i64,
    exp: i64,
    iss: String,
}

// There are other fields but token is the only one we care about
#[derive(Debug, Deserialize)]
struct GithubTokenResponse {
    token: String,
}

#[derive(Default)]
pub struct GithubTokenManager {
    tokens: Mutex<HashMap<i64, (String, SystemTime)>>,
    client: Client,
}

impl GithubTokenManager {
    pub async fn get_token(&self, installation_id: i64) -> anyhow::Result<String> {
        // We don't want to hold the lock across the network call
        {
            let tokens = self.tokens.lock().await;
            if let Some((token, expiry)) = tokens.get(&installation_id) {
                if expiry.duration_since(SystemTime::now())? > EXPIRATION_BUFFER {
                    return Ok(token.clone());
                }
            }
        }

        let new_token = self.fetch_new_token(installation_id).await?;
        let mut tokens = self.tokens.lock().await;
        tokens.insert(installation_id, new_token.clone());

        Ok(new_token.0)
    }

    async fn fetch_new_token(&self, installation_id: i64) -> anyhow::Result<(String, SystemTime)> {
        let jwt = self.create_jwt()?;

        let response: GithubTokenResponse = post(
            &self.client,
            format!(
                "https://api.github.com/app/installations/{}/access_tokens",
                installation_id
            ),
            &jwt,
        )
        .await?;

        // GitHub app installation access tokens live for 1 hour
        let expiry = SystemTime::now() + TTL;
        Ok((response.token, expiry))
    }

    fn create_jwt(&self) -> anyhow::Result<String> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64;
        let claims = Claims {
            iat: now - 60,
            exp: now + 300,
            iss: env::var("PUBLIC_GITHUB_APP_CLIENT_ID")?,
        };

        // Fly messes up the formatting of our secret
        let private_key = env::var("GITHUB_APP_PRIVATE_KEY")?.replace("\\n", "\n");
        let encoding_key = EncodingKey::from_rsa_pem(private_key.as_bytes())?;

        let header = Header::new(Algorithm::RS256);
        Ok(encode(&header, &claims, &encoding_key)?)
    }
}

async fn post<T, U>(client: &Client, url: U, access_token: &str) -> Result<T, reqwest::Error>
where
    T: DeserializeOwned,
    U: IntoUrl,
{
    client
        .post(url)
        .bearer_auth(access_token)
        .header("User-Agent", "sundry")
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .send()
        .await?
        .json::<T>()
        .await
}
