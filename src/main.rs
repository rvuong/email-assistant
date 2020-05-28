#[macro_use]
extern crate log;
extern crate env_logger;

use std::env;
use std::error::Error;

mod reader;
mod receipt;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let debug: bool = env::var("DEBUG")
        .expect("Missing or invalid env var: DEBUG")
        .parse()
        .unwrap();

    let imap_host = env::var("IMAP_HOST").expect("Missing or invalid env var: IMAP_HOST");
    let imap_username =
        env::var("IMAP_USERNAME").expect("Missing or invalid env var: IMAP_USERNAME");
    let imap_password =
        env::var("IMAP_PASSWORD").expect("Missing or invalid env var: IMAP_PASSWORD");
    let imap_port: u16 = env::var("IMAP_PORT")
        .expect("Missing or invalid env var: IMAP_PORT")
        .to_string()
        .parse()
        .unwrap();

    let messages =
        reader::fetch_inbox(imap_host, imap_username, imap_password, imap_port, debug).unwrap();
    if messages.len().eq(&0) {
        // No messages need to be processed
        return Ok(());
    }

    // Builds the transport mailer
    let smtp_options = receipt::SmtpOptions {
        host: env::var("SMTP_HOST").expect("Missing or invalid env var: SMTP_HOST"),
        username: env::var("SMTP_USERNAME").expect("Missing or invalid env var: SMTP_USERNAME"),
        password: env::var("SMTP_PASSWORD").expect("Missing or invalid env var: SMTP_PASSWORD"),
        port: env::var("SMTP_PORT")
            .expect("Missing or invalid env var: SMTP_PORT")
            .to_string()
            .parse()
            .unwrap(),
    };

    // Loops through IMAP messages
    for message in messages.iter() {
        let sender = format!("{}", message.sender);
        let subject = format!("Re: {}", message.subject);
        let recipient = if debug {
            "remy.vuong@davidson.fr".to_string()
        } else {
            format!("{}", sender)
        };

        let references = if message.references.eq("") {
            format!("<{id}>", id = message.message_id)
        } else {
            [
                format!("<{ref}>", ref = message.references),
                format!("<{id}>", id = message.message_id),
            ]
            .join(" ")
        };
        let receipt_message = receipt::ReceiptMessage {
            sender: env::var("SMTP_SENDER").expect("Missing or invalid env var: SMTP_SENDER"),
            recipient,
            subject,
            body: "Your request was received and will be processed shortly.\r\n\r\nBest regards\r\n\r\n--\r\nAssistant".to_string(),   // TODO Should be set dynamically
            in_reply_to: format!("<{}>", message.message_id),
            references,
        };
        let smtp = receipt::SmtpOptions {
            host: smtp_options.host.clone(),
            username: smtp_options.username.clone(),
            password: smtp_options.password.clone(),
            port: smtp_options.port.clone(),
        };
        receipt::send(smtp, receipt_message);
    }

    Ok(())
}
