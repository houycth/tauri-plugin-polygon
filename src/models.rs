use serde::{Deserialize, Serialize};

use crate::error::Error;

pub(crate) type CommandResult = Result<Response, Response>;

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Response {
    ok: bool,
    error: Option<String>,
}

impl Response {
    pub(crate) fn ok() -> CommandResult {
        Ok(Self {
            ok: true,
            error: None,
        })
    }
    pub(crate) fn err(error: Error) -> CommandResult {
        Err(Self {
            ok: false,
            error: Some(error.to_string()),
        })
    }
}
