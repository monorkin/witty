use serde_json::Value;

///
/// Represents all possbile unexpected states
///
#[derive(Debug)]
pub struct ExecutionError {
    pub message: String,
    pub code: u32
}

///
/// Handles comunication between the application and Wit.ai
///
#[derive(Debug)]
pub struct Client {
    pub token: String
}

impl Client {
    ///
    /// Builds and returns a new Client struct with the passed token and
    /// actions. If the passed actions are invalid `None` is returned, else an
    /// `Option` containing the Client is returned.
    ///
    /// Note that the token is used to comunicate with Wit.ai servers and has
    /// to be obtained by them. To lear how to get your token visit
    /// `https://wit.ai/docs/quickstart`.
    /// The token can be found in your application's settings on wit.ai.
    ///
    pub fn new(token: &str) -> Client {
        Client {
            token: token.to_string()
        }
    }

    ///
    /// Used to get the exact meaning of a sentance after it's processed by
    /// wit.ai. If the server can't be reached, or the token is invalid `None`
    /// is returned.
    ///
    /// This is useful for probing how wit.ai will respond to a message.
    ///
    pub fn message(&self, message: &str) ->
        Result<Value, super::http::HttpError>
    {
        let params = json!({
            "q": message.to_string()
        });

        super::http::request(
            self.token.to_owned(),
            super::http::Method::Get,
            "/message".to_string(),
            Some(params),
            None
        )
    }
}
