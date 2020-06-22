/**
 * The purpose of this module is to manage connexions to API EOS.
 */
use reqwest::header::{HeaderMap, AUTHORIZATION};
use reqwest::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct EosAuthentication {
    token: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct EosUser {
    id: u32,
}

/**
 * Get the EOS user id given her/his email
 */
pub fn user_id(token: String, email: String, url: &str) -> Result<u32, Error> {
    let client = reqwest::blocking::Client::new();
    let login_url = format!("{}api/getWorkerbyEmail/{}/exec", url, email);

    let mut headers = HeaderMap::new();
    let authorization_value = format!("Bearer {}", token);
    headers.insert(AUTHORIZATION, authorization_value.parse().unwrap());
    let res = client.get(login_url.as_str()).headers(headers).send()?;

    match res.status() {
        reqwest::StatusCode::OK => {}
        s => {
            debug!("Unknown user: {}", s);

            return Ok(0);
        }
    }

    let body = res.text().unwrap();
    let user: EosUser = serde_json::from_str(body.as_str()).unwrap();

    Ok(user.id)
}
