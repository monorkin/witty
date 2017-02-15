use serde_json::{Value, Map};
use std::collections::BTreeMap;

///
/// Error object that represents errors related to the input actions
///
pub struct InvalidActionsError {
    message: String
}

///
/// Handles comunication between the application and Wit.ai
///
pub struct Client {
    token: String,
    actions: BTreeMap<String, fn(Map<String, Value>, Map<String, Value>)>
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
    pub fn new(
        token: &str,
        actions: BTreeMap<String, fn(Map<String, Value>, Map<String, Value>)>
    ) -> Result<Client, InvalidActionsError> {
        if !Client::validate_actions(&actions) {
            return Err(
                InvalidActionsError {
                    message: "The given methods are invalid. Please check if
                        you passed a `send` method."
                }
            );
        }

        Ok(
            Client {
                token: token.to_string(),
                actions: actions
            }
        )
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
        let mut params: Map<String, Value> = Map::new();
        params.insert("q".to_string(), Value::String(message.to_string()));

        super::http::request(
            self.token.to_owned(),
            super::http::Method::Get,
            "/message".to_string(),
            Some(params),
            None
        )
    }

    ///
    /// Used to fetch the next response.
    /// To get all responses you need to successively call this method until it
    /// returns a response with type `stop`.
    /// This method returns `None` if the server is unreachable or the token is
    /// invalid.
    ///
    /// This is useful if your bot has multiple responses or branching
    /// responses.
    ///
    pub fn converse(
        &self, session_id: &str, message: &str, context: Value, reset: bool
    ) -> Result<Value, super::http::HttpError>
    {
        Ok(Map::new())
    }

    ///
    /// Executes apropriate methods based on the input text.
    /// This method combines the message and converse methods together. It
    /// iterates over the responses and calls each method asociated with it to
    /// execute logic.
    ///
    pub fn run_actions(
        &self, session_id: &str, message: &str, context: Map<String, Value>
    )
    {
        // TODO
    }


    ///
    /// Checks if the assigned actions are valid.
    /// If they are it returns `true`, else it returns `false`.
    ///
    /// A valid list of actions needs to have an action called `send`.
    ///
    fn validate_actions(
        actions: &BTreeMap<String, fn(Map<String, Value>, Map<String, Value>)>
    ) -> bool {
        match actions.get("send") {
            Some(function) => return true,
            _ => return false
        }

        false
    }
}
