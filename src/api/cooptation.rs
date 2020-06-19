/**
 * The purpose of this module is to manage connexions to API Cooptation.
 */
use crate::api::profiling;
use crate::service::inbox::Email;

use reqwest::blocking::multipart;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE, USER_AGENT};
use reqwest::Error;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct EosAuthentication {
    token: String,
}

/**
 * @todo This should be refactored with EOS authenticate() function
 *
 * POST request with url, login, & password. Should return a JSON-encapsulated auth token
 */
pub fn authenticate(username: &str, password: &str, url: &str) -> Result<String, Error> {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("reqwest"));
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json")); // Mandatory

    let client = reqwest::blocking::Client::new();
    let login_url = format!("{}api/login_check", url);
    let request_body = format!(
        "{{ \"_username\": \"{}\", \"_password\": \"{}\" }}",
        username, password
    );
    let res = client
        .post(login_url.as_str())
        .headers(headers)
        .body(request_body)
        .send()?;

    // Handles errors
    match res.status() {
        reqwest::StatusCode::OK => debug!("Authentication successful"),
        s => {
            debug!("Authentication error: {}", s);

            return Ok(String::from(""));
        }
    }

    // Token deserialization
    let body = res.text().unwrap();
    let j: EosAuthentication = serde_json::from_str(body.as_str()).unwrap();

    Ok(j.token.to_string())
}

/**
 * @todo
 *
 * POST data should be set dynamically
 */
pub fn add(email: &Email, user_id: u32, token: String, url: &str) -> Result<(), Error> {
    let _client = reqwest::blocking::Client::new();

    // POST a cooptation
    let _url = format!("{}api/test/cooptation", url);
    let mut headers = HeaderMap::new();
    let authorization_value = format!("Bearer {}", token);
    headers.insert(AUTHORIZATION, authorization_value.parse().unwrap());

    let user_id = format!("{}", user_id);
    let root_path = env::current_dir().unwrap();
    let attachment_path = format!("{}/{}", root_path.display(), email.attachment);

    let _branch = profiling::get(email, "branch").unwrap();
    let _firstname = profiling::get(email, "firstname").unwrap();
    let _lastname = profiling::get(email, "lastname").unwrap();
    let _email = profiling::get(email, "email").unwrap();
    let _relation = profiling::get(email, "relation").unwrap();
    let _phone = profiling::get(email, "phone").unwrap();

    let form = multipart::Form::new()
        .text("branch", _branch) // TODO Recommended branch IDs (","-separated list)
        .text("firstname", _firstname) // TODO Candidate firstname, should be determined by Profiling
        .text("lastname", _lastname) // TODO Candidate lastname, should be determined by Profiling
        .text("email", _email) // TODO Candidate email, should be determined by Profiling
        .text("relation", _relation) // TODO Either text/plain or html content of the input email
        .text("phone", _phone) // TODO Candidate phone number, should be determined by Profiling
        .text("userid", user_id)
        .file("cv", attachment_path)
        .unwrap();

    let debug: bool = env::var("DEBUG")
        .expect("Missing or invalid env var: DEBUG")
        .parse()
        .unwrap();

    if !debug {
        let _res = _client
            .post(_url.as_str())
            .headers(headers)
            .multipart(form)
            .send()
            .unwrap();
    }

    Ok(())
}
