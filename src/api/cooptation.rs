/**
 * The purpose of this module is to manage connexions to API Cooptation.
 */
use crate::api::profiling;
use crate::service::inbox::Email;

use reqwest::blocking::multipart;
use reqwest::header::{HeaderMap, AUTHORIZATION};
use reqwest::Error;
use std::env;

/**
 * @todo
 *
 * POST data should be set dynamically
 */
pub fn add(email: &Email, user_id: u32, token: String, url: &str) -> Result<(), Error> {
    let client = reqwest::blocking::Client::new();

    // POST a cooptation
    let url = format!("{}api/test/cooptation", url);
    let mut headers = HeaderMap::new();
    let authorization_value = format!("Bearer {}", token);
    headers.insert(AUTHORIZATION, authorization_value.parse().unwrap());

    let user_id = format!("{}", user_id);
    let root_path = env::current_dir().unwrap();
    let attachment_path = format!("{}/{}", root_path.display(), email.attachment);

    let branch = profiling::get(email, "branch").unwrap();
    let firstname = profiling::get(email, "firstname").unwrap();
    let lastname = profiling::get(email, "lastname").unwrap();
    let email_address = profiling::get(email, "email").unwrap();
    let relation = profiling::get(email, "relation").unwrap();
    let phone = profiling::get(email, "phone").unwrap();

    let form = multipart::Form::new()
        .text("branch", branch) // TODO Recommended branch IDs (","-separated list)
        .text("firstname", firstname) // TODO Candidate firstname, should be determined by Profiling
        .text("lastname", lastname) // TODO Candidate lastname, should be determined by Profiling
        .text("email", email_address) // TODO Candidate email, should be determined by Profiling
        .text("relation", relation) // TODO Either text/plain or html content of the input email
        .text("phone", phone) // TODO Candidate phone number, should be determined by Profiling
        .text("userid", user_id)
        .file("cv", attachment_path)
        .unwrap();

    let _res: reqwest::blocking::Response = client
        .post(url.as_str())
        .headers(headers)
        .multipart(form)
        .send()
        .unwrap();
    // TODO Assert result status() and manage potential errors

    Ok(())
}
