extern crate base64;
extern crate native_tls;
extern crate regex;

use std::error::Error;
use std::fs;

use base64::decode;
use native_tls::TlsConnector;
use regex::Regex;

const DOUBLE_CRLF: &str = "\r\n\r\n";

pub struct Email {
    pub header: String,
    pub body: Vec<EmailPart>,
    pub message_id: String,
    pub references: String,
    pub sender: String,
    pub subject: String,
}

pub struct EmailPart {
    pub header: String,
    pub content: String,
}

pub fn fetch_inbox(
    host: String,
    username: String,
    password: String,
    port: u16,
    debug: bool,
) -> Result<Vec<self::Email>, Box<dyn Error>> {
    let mut messages: Vec<self::Email> = Vec::new();

    let tls = TlsConnector::builder().build().unwrap();

    // we pass in the domain twice to check that the server's TLS
    // certificate is valid for the domain we're connecting to.
    let client = imap::connect_starttls((host.as_str(), port), host.as_str(), &tls).unwrap();
    info!("Connected to {:?}:{:?}", host, port);

    // the client we have here is unauthenticated.
    // to do anything useful with the e-mails, we need to log in
    let mut imap_session = client
        .login(username.as_str(), password.as_str())
        .map_err(|e| e.0)?;
    info!("Logged in as \"{}\"", username.as_str());

    // we want to fetch the first email in the INBOX mailbox
    imap_session.select("INBOX")?;
    info!("Inbox \"INBOX\" selected");

    // Search & filter the messages in the inbox
    let message_indexes = imap_session.search("UNSEEN")?;
    for message_index in &message_indexes {
        let fetches = imap_session.fetch(format!("{}", message_index), "RFC822")?;
        let message = if let Some(m) = fetches.iter().next() {
            m
        } else {
            return Ok(messages);
        };

        let body = message.body().expect("Missing or invalid body");
        let body = std::str::from_utf8(body)
            .expect("Not UTF-8 valid")
            .to_string();
        let email = self::get_email(body);

        info!(
            "Message #{} successfully fetched (\"{}\", from: \"{}\")",
            message_index, email.subject, email.sender
        );

        // TODO Accurate filtering (cooptation related, etc)
        // if email.subject.contains("cooptation") || email.body.contains("cooptation") {
        if email.subject.to_ascii_lowercase().contains("cooptation") {
            messages.push(email);
        }

        // Sets messages as unread again
        if debug {
            debug!("Message #{} set back as unseen", message_index);
            imap_session.store(format!("{}", message_index), "-FLAGS (\\Seen)")?;
        }
    }

    // be nice to the server and log out
    imap_session.logout()?;

    Ok(messages)
}

/**
 * Returns an email struct given the message data
 */
fn get_email(data: String) -> Email {
    // Read & identify inner content
    let mut email_header = "".to_string();
    let mut email_body = Vec::new();

    let v = data.splitn(2, DOUBLE_CRLF);
    let mut is_header: bool = true;
    for r in v {
        if is_header {
            email_header = r.to_string();
            is_header = false;
        } else {
            email_body = self::get_body_parts(email_header.to_string(), r.to_string());
        }
    }

    let message_id = self::get_email_attribute(email_header.to_string(), "Message-ID");
    let sender = self::get_email_attribute(email_header.to_string(), "From");
    let subject = self::get_email_attribute(email_header.to_string(), "Subject");
    let references = self::get_email_attribute(email_header.to_string(), "References");

    Email {
        header: email_header,
        body: email_body,
        message_id,
        sender,
        subject,
        references,
    }
}

/**
 * Returns the simplified value for the given header's key
 * If none, returns a blank String ("".to_string())
 */
fn get_email_attribute(header: String, key: &str) -> String {
    // Get simple headers
    let headers_regex = Regex::new(r"(?m)^([a-zA-Z-]+): ([^\r]*)\r?$").unwrap();
    for row in headers_regex.captures_iter(header.as_str()) {
        if (&row[1]).eq(key) {
            return get_simple_value((&row[2]).to_string());
        }
    }

    "".to_string()
}

/**
 * Get the core value for <> encapsulated data
 *
 * eg.:
 * "John Doe <john.doe@foo.bar>" would return "john.doe@foo.bar"
 * "Message-ID: <989bac4f-277b-84a7-5601-8975e35dfba5@davidson.fr>" would return "989bac4f-277b-84a7-5601-8975e35dfba5@davidson.fr"
 */
fn get_simple_value(complex_value: String) -> String {
    let core_regex = Regex::new(r"^([^<>]*<)?(?P<value>[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*).*$").unwrap();
    let simple_value = core_regex.replace(complex_value.as_str(), "$value");

    simple_value.to_string()
}

/**
 *
 */
fn get_simpler_body(body: String, end_boundary: &str) -> String {
    let tmp = "".to_string();

    let v = body.split(end_boundary);
    for i in v {
        return i.to_string();
    }

    tmp.to_string()
}

/**
 *
 */
fn get_body_parts(headers: String, body: String) -> Vec<EmailPart> {
    let mut parts: Vec<EmailPart> = Vec::new();

    let boundary = self::get_boundary(headers.to_string());
    let end_boundary = format!("\r\n--{}--\r\n", boundary);
    // Excludes the ending boundary and everything after
    let body = self::get_simpler_body(body, end_boundary.as_str());

    let body_parts = body.split(boundary.as_str());

    // Loops through the email's body parts
    for part in body_parts {
        // Hard-coded continue statements
        if part.contains("This is a multi-part message in MIME format.")
            || part.eq(format!("--{}", DOUBLE_CRLF).as_str())
        {
            continue;
        }

        let part_elements = part.splitn(2, DOUBLE_CRLF);
        let mut is_header = true;
        let mut tmp_header = "".to_string();
        let mut tmp_content_disposition;
        let mut tmp_content = "".to_string();
        let mut is_attachment: bool = false;
        let mut attachment_filename = "".to_string();
        for part_element in part_elements {
            if is_header {
                tmp_header = part_element.to_string();
                tmp_content_disposition =
                    self::get_email_attribute(tmp_header.to_string(), "Content-Disposition");
                is_attachment = tmp_content_disposition.contains("attachment");
                if is_attachment {
                    attachment_filename = self::get_attachment_filename(tmp_header.to_string());
                }

                is_header = false;
            } else {
                // Attachments handling
                if is_attachment {
                    let tmp_attachment = part_element.replace("\r\n", "");
                    let binary = decode(tmp_attachment).unwrap();
                    // Use the accurate filename (and path)
                    let file_path = format!("var/tmp/{}", attachment_filename);
                    match fs::write(file_path, binary) {
                        Err(e) => println!("{:?}", e),
                        _ => (),
                    }
                }
                tmp_content = part_element.to_string();
            }
        }

        let email_part = EmailPart {
            header: tmp_header,
            content: tmp_content,
        };
        parts.push(email_part);
    }

    parts
}

/**
 *
 */
fn get_attachment_filename(data: String) -> String {
    let data = data.replace("\r\n", " ");
    let re = Regex::new(".*filename=\"(?P<filename>.*)\".*").unwrap();
    let filename = re.replace(data.as_str(), "$filename");

    filename.to_string()
}

/**
 * Returns the email body boundaries identifier
 */
fn get_boundary(input: String) -> String {
    let tmp: Vec<&str> = input.split(DOUBLE_CRLF).collect();
    let first = tmp.iter().next().unwrap();
    let first = first.replace("\r\n", " ");
    let bound_re = Regex::new(r"^.*boundary=.(?P<boundary>[-]+[a-zA-Z0-9]+)..*$").unwrap();
    let boundary_delimiter = bound_re.replace(first.as_str(), "$boundary");

    boundary_delimiter.to_string()
}
