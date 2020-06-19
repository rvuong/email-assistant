use crate::service::inbox::Email;
use std::io::Error;

/**
 * @todo
 */
pub fn get(_email: &Email, key: &str) -> Result<String, Error> {
    let result = match key {
        "branch" => String::from("1"),
        "firstname" => String::from("John"),
        "lastname" => String::from("Doe"),
        "email" => String::from("me@domain.tld"),
        "relation" => String::from("--Relation"),
        "phone" => String::from("01 02 03 04 05"),
        _ => {
            warn!("Invalid parameter name required: {}", key);

            String::from("")
        }
    };

    Ok(result)
}
