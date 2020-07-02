use crate::service::inbox::Email;
use regex::Regex;
use reqwest::header::CONTENT_TYPE;
use std::io::Error;

/**
 * @todo
 */
pub fn get(email: &Email, key: &str) -> Result<String, Error> {
    let result = match key {
        "branch" => String::from("Davidson Paris"),
        "firstname" => String::from("John"),
        "lastname" => String::from("Doe"),
        "email" => String::from("me@domain.tld"),
        "relation" => get_body_text(email).unwrap(),
        "phone" => String::from("01 02 03 04 05"),
        "purpose" => get_purpose(email).unwrap(),
        _ => {
            warn!("Invalid parameter name required: {}", key);

            String::from("")
        }
    };

    Ok(result)
}

/**
 * @todo
 *
 * Returns the text/plain or text/html message body if any was found.
 * Should be replaced by another method with more consistency.
 */
fn get_body_text(email: &Email) -> Result<String, Error> {
    for item in email.body.iter().next() {
        if item.is_attachment {
            continue;
        }

        let headers_regex = Regex::new(r"(?m)^([a-zA-Z-]+): ([^\r]*)\r?$").unwrap();
        for row in headers_regex.captures_iter(item.header.as_str()) {
            if (&row[1]).to_ascii_lowercase().eq(CONTENT_TYPE.as_str()) {
                // If the Content-Type is "text/plain" or "text/html"
                if (&row[2]).contains("text/plain") {
                    return Ok(item.content.to_string());
                } else if (&row[2]).contains("multipart/alternative") {
                    return Ok(
                        "// TODO text/html message should be converted into text/plain".to_string(),
                    );
                }
            }
        }
    }

    return Ok("// TODO Message not found".to_string());
}

/**
 * @todo
 */
fn get_purpose(email: &Email) -> Result<String, Error> {
    if email.subject.to_ascii_lowercase().contains("cooptation") {
        return Ok("cooptation".to_string());
    }

    Ok("".to_string())
}
