use serde_json::Value;
use serde_json;
use hyper::client::*;
use hyper::status::StatusCode;
use hyper::header::*;
use hyper::mime::*;
use std::io::Read;
use serde_urlencoded;

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
    pub message: String,
    pub status: u16,
    pub code: u16
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

    let mut headers = Headers::new();
    let authorization_header = format!("Bearer {}", &token[..]);
    headers.set(Authorization(authorization_header));
    let accept_header = format!("application/vnd.wit.{}+json", API_VERSION);
    let accept_header =
        Header::parse_header(&[accept_header.as_bytes().to_vec()][..])
        .unwrap_or(Accept(vec![]));
    headers.set(accept_header);
    headers.set(
        ContentType(Mime(TopLevel::Application, SubLevel::Json, vec![]))
    );

    let request = request.headers(headers);

    let response = match request.send() {
        Ok(response) => response,
        Err(_error) => {
            return Err(
                HttpError {
                    message: "Could not connect to server".to_string(),
                    status: 0,
                    code: 100
                }
            )
        }
    };

    if response.status != StatusCode::Ok {
        let reason = format!(
            "Server responded with error: {}",
            response.status
        );


        return Err(
            HttpError {
                message: reason,
                status: response.status.to_u16(),
                code: 101
            }
        );
    };

    Ok(deserialize_response(response))
}

///
/// Given a path and params it builds a fully qualified URL
///
fn build_url(path: String, params: Option<Value>) -> String {
    let mut url = API_HOST.to_string();

    url.push_str(&path);

    let params = match params {
        Some(json) => json,
        _ => json!({})
    };
    let url_encoded_params = match serde_urlencoded::to_string(&params) {
        Ok(encoded_params) => encoded_params,
        Err(_error) => "".to_string()
    };
    url.push_str(&url_encoded_params);

    url
}


///
/// Converts the input map into a JSON serialized string of the input map
///
fn build_body(payload: Value) -> String {
    payload.to_string()
}

///
/// Converts the raw response string to a map
///
fn deserialize_response(response: Response) -> Value {
    let mut response = response;

    if response.status != StatusCode::Ok {
        return json!({});
    }

    let mut body = String::new();

    match response.read_to_string(&mut body) {
        Ok(body) => body,
        Err(_error) => return json!({})
    };

    println!("body: {}", &body[..]);

    match serde_json::from_str(&body[..]) {
        Ok(body) => body,
        Err(_error) => json!({})
    }
}
