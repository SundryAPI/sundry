use serde::{Deserialize, Serialize};

pub mod query;
pub mod return_data;
pub mod utils;

use query::Query;
use return_data::ReturnData;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "action")]
pub enum Step {
    Query(Query),
    Return(ReturnData),
}

#[cfg(feature = "integration")]
impl Step {
    pub fn as_query(&self) -> Option<&Query> {
        match self {
            Step::Query(query) => Some(query),
            Step::Return(_) => None,
        }
    }

    pub fn as_return(&self) -> Option<&ReturnData> {
        match self {
            Step::Query(_) => None,
            Step::Return(ret) => Some(ret),
        }
    }
}
