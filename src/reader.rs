extern crate native_tls;
extern crate regex;

use std::error::Error;

use imap::types::Fetch;
use native_tls::TlsConnector;
use regex::Regex;

pub struct Message {
    pub sender: String,
    pub subject: String,
    pub body: String,
    pub attachment: String,
    pub message_id: String,
    pub references: String,
}

pub fn fetch_inbox(
    host: String,
    username: String,
    password: String,
    port: u16,
    debug: bool,
) -> Result<Vec<self::Message>, Box<dyn Error>> {
    let mut messages: Vec<self::Message> = Vec::new();

    let tls = TlsConnector::builder().build().unwrap();

    // we pass in the domain twice to check that the server's TLS
    // certificate is valid for the domain we're connecting to.
    let client = imap::connect_starttls((host.as_str(), port), host.as_str(), &tls).unwrap();
    debug!("Connected to {:?}:{:?}", host, port);

    // the client we have here is unauthenticated.
    // to do anything useful with the e-mails, we need to log in
    let mut imap_session = client
        .login(username.as_str(), password.as_str())
        .map_err(|e| e.0)?;
    debug!("Session logged in as \"{}\"", username.as_str());

    // we want to fetch the first email in the INBOX mailbox
    imap_session.select("INBOX")?;
    debug!("Inbox \"INBOX\" selected");

    // Search & filter the messages in the inbox
    let message_ids = imap_session.search("UNSEEN")?;
    for message_id in &message_ids {
        let fetches = imap_session.fetch(format!("{}", message_id), "RFC822")?;
        let message = if let Some(m) = fetches.iter().next() {
            m
        } else {
            return Ok(messages);
        };

        let data = self::get_message(message);
        info!(
            "Message #{} was fetched (sender: \"{}\", subject: \"{}\")",
            message_id, data.sender, data.subject
        );

        // TODO Accurate filtering
        if data.subject.contains("cooptation") {
            messages.push(data);
        }

        // Sets messages as unread again
        if debug {
            debug!("Set message #{} back to Unseen status", message_id);
            imap_session.store(format!("{}", message_id), "-FLAGS (\\Seen)")?;
        }
    }

    // be nice to the server and log out
    imap_session.logout()?;

    Ok(messages)
}

fn get_message(message: &Fetch) -> self::Message {
    // TODO Get the message ID (for the reply)

    let mut m = self::Message {
        sender: "".to_string(),
        subject: "".to_string(),
        body: "".to_string(),       // TODO
        attachment: "".to_string(), // TODO
        message_id: "".to_string(),
        references: "".to_string(),
    };

    let body = message.body().expect("Missing or invalid body");
    let body = std::str::from_utf8(body)
        .expect("Not UTF-8 valid")
        .to_string();

    // Split the header into several rows
    let body_vec = body.split("\r\n");
    let regex =
        Regex::new(r"^(?P<type>Subject|From|Date|Message-ID|References): (?P<data>.*)$").unwrap();

    for body_row in body_vec {
        if regex.is_match(body_row) {
            let row_type = regex.replace(body_row, "$type");
            let row_data = regex.replace(body_row, "$data");

            if row_type.eq("Subject") {
                // Extract the email subject
                m.subject = format!("{}", row_data);
            } else if row_type.eq("From") {
                // Extract the sender's email address
                let email_regex = Regex::new(r"^(From: )?([^<>]*<)?(?P<email>[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*).*$").unwrap();
                let sender = email_regex.replace(body_row, "$email");
                m.sender = format!("{}", sender);
            } else if row_type.eq("Message-ID") {
                let message_id_regex =
                    Regex::new(r"^Message-ID: <?(?P<message_id>[a-zA-Z0-9@.-]+)>?").unwrap();
                let message_id = message_id_regex.replace(body_row, "$message_id");
                m.message_id = format!("{}", message_id);
            } else if row_type.eq("References") {
                let references_regex =
                    Regex::new(r"^References: <?(?P<references>[a-zA-Z0-9@.-]+)>?").unwrap();
                let references = references_regex.replace(body_row, "$references");
                m.references = format!("{}", references);
            }
        }

        // TODO Extract the email body text
        // TODO Extract the 1st attachment (CV)
    }

    return m;
}
