use serde_json::{Value, Map};
use std::collections::BTreeMap;

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
    pub token: String,
    pub actions: BTreeMap<String, fn(Value) -> Value>,
    pub send_method: fn(Value, Value)
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
        actions: BTreeMap<String, fn(Value) -> Value>,
        send_method: fn(Value, Value)
    ) -> Client {
        Client {
            token: token.to_string(),
            actions: actions,
            send_method: send_method
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
        let mut params = json!({
            "q": message.to_string(),
            "session_id": session_id
        });

        if reset {
            params["reset"] = json!(true);
        }

        super::http::request(
            self.token.to_owned(),
            super::http::Method::Post,
            "/converse".to_string(),
            Some(params),
            Some(context)
        )
    }

    ///
    /// Executes apropriate methods based on the input text.
    /// This method combines the message and converse methods together. It
    /// iterates over the responses and calls each method asociated with it to
    /// execute logic.
    ///
    pub fn run_actions(
        &self, session_id: &str, message: &str, context: Value,
        max_steps: Option<i8>
    ) -> Result<Value, ExecutionError>
    {
        // Persume a defualt value of 5 if no inpt given
        let mut steps = max_steps.unwrap_or(5);

        // Exit the recursion if the maximum number of steps was exceded
        if steps < 0 {
            return Err(ExecutionError {
                message: "Max steps reached, stopping.".to_string(),
                code: 101
            });
        }

        // Get a response from the server
        let json_response = self.converse(
            session_id.clone(), message.clone(), context.clone(), false
        );

        let json = match json_response {
            Ok(json) => json,
            Err(error) => {
                let reason = format!(
                    "Could not connect to API. {}",
                    error.message
                );

                return Err(ExecutionError { message: reason, code: 106 });
            }
        };

        // Exit if the API didn't specify a response type
        if json.get("type") == None {
            return Ok(context);
        }

        // Fix backwards compatibility issues
        let json = Client::make_response_backwards_compatible(json);

        // Exit if the API encountered an error
        if Client::unwrap_to_string(json.get("type")) == "error" {
            return Err(ExecutionError {
                message: "The API responded with an error.".to_string(),
                code: 102
            });
        }

        // Exit if the API instructed us to stop
        if Client::unwrap_to_string(json.get("type")) == "stop" {
            return Ok(context);
        }

        // Build request object
        let request = json!({
            "session_id": session_id,
            "context": context,
            "text": message
            // "entities": json.get("enteties").unwrap_or(json!({}))
        });

        // Build returned context object
        let mut new_context = context;

        // Run action for response type
        if Client::unwrap_to_string(json.get("type")) == "msg" {
            // The API wants us to send out the message.
            // Therefore we build a response object and pass both the request
            // and response objects to the `send` method to handle the actual
            // message sending.
            let response = json!({
                "text": Client::unwrap_to_string(json.get("msg")),
                "quickreplies":
                    Client::unwrap_to_string(json.get("quickreplies"))
            });

            (self.send_method)(request, response);
        }
        else if Client::unwrap_to_string(json.get("type")) == "action" {
            // The API wants us to trigger an action.
            let action_name = match json.get("action") {
                Some(name) => name.as_str().unwrap_or(""),
                _ => {
                    let reason =
                        "The API didn't specify an action to run.".to_string();
                    return Err(ExecutionError { message: reason, code: 103 });
                }
            };

            let action = match self.actions.get(action_name) {
                Some(function) => function,
                _ => {
                    let reason = format!(
                        "The API wanted to run non-existing method `{}`.",
                        action_name
                    );

                    return Err(ExecutionError { message: reason, code: 104 });
                }
            };

            new_context = action(request);
        }
        else {
            return Err(ExecutionError {
                message: "The API responded with an unknown type.".to_string(),
                code: 105
            });
        }

        self.run_actions(session_id, message, new_context, Some(steps - 1))
    }

    ///
    /// Resolves backwards compatibility issues with other API versions
    ///
    fn make_response_backwards_compatible(old_json: Value) -> Value {
        let mut json = old_json;

        let response_type = Client::unwrap_to_string(json.get("type"));

        if response_type == "merge" {
            json["type"] = json!("action");
            json["action"] = json!("merge");
        }

        json
    }

    ///
    /// Given a Value it return it as a String
    ///
    fn unwrap_to_string(value: Option<&Value>) -> String {
        let value = match value {
            Some(value) => value,
            _ => return "".to_string()
        };

        value.as_str().unwrap_or("").to_string()
    }
}
