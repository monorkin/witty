use serde_json::{Value, Map};
use hyper::client::*;

///
/// Contains the base URL to access the API
///
const API_HOST: &'static str = "https://api.wit.ai";

///
/// Contains the version of the API to use. This is required.
///
const API_VERSION: &'static str = "20160516";

///
/// Describes HTTP access methods
///
pub enum Method {
    Get,
    Post
}

///
/// Represents an error in somunicating with the server
///
#[derive(Debug)]
pub struct HttpError {
    message: String,
    status: u16
}

///
/// Handles comunication with the wit.ai servers.
/// The result is a decoded JSON object.
///
pub fn request(
    token: String, method: Method, path: String,
    params: Option<Value>, payload: Option<Value>

) -> Result<Value, HttpError>
{
    let url = &*build_url(path, params);
    let client = Client::new();
    let body = &*build_body(payload.unwrap_or(json!({})));

    let request = match method {
        Method::Get => client.get(&*url),
        Method::Post => client.post(&*url)
    };

    let request = request.body(body);

    let response = request.send();

    match response {
        Ok(response) => Ok(deserialize_response(response)),
        Err(_error) => Err(
            HttpError {
                message: "Could not connect to server".to_string(),
                status: 0
            }
        )
    }
}

///
/// Given a path and params it builds a fully qualified URL
///
fn build_url(path: String, params: Option<Value>) -> String {
    let mut url = API_HOST.to_string();

    url.push_str(&path);

    url
}


///
/// Converts the input map into a JSON serialized string of the input map
///
fn build_body(payload: Value) -> String {
    "".to_string()
}

///
/// Converts the raw response string to a map
///
fn deserialize_response(response: Response) -> Value {
    json!({})
}
