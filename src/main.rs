#[macro_use]
extern crate log;
extern crate env_logger;

use std::env;
use std::error::Error;

mod api;
mod service;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let debug: bool = env::var("DEBUG")
        .expect("Missing or invalid env var: DEBUG")
        .parse()
        .unwrap();

    // IMAP env vars
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
        service::inbox::parse(imap_host, imap_username, imap_password, imap_port, debug).unwrap();
    if messages.len().eq(&0) {
        // No messages need to be processed
        return Ok(());
    }

    // EOS authentication is required to get the sender's EOS ID
    let eos_url = env::var("EOS_URL").expect("Missing or invalid env var: EOS_URL");
    let eos_username = env::var("EOS_USERNAME").expect("Missing or invalid env var: EOS_USERNAME");
    let eos_password = env::var("EOS_PASSWORD").expect("Missing or invalid env var: EOS_PASSWORD");
    let eos_token = api::eos::authenticate(
        eos_username.as_str(),
        eos_password.as_str(),
        eos_url.as_str(),
    )
    .unwrap();

    // API Cooptation authentication is required to post new cooptations
    let cooptation_url =
        env::var("COOPTATION_URL").expect("Missing or invalid env var: COOPTATION_URL");
    let cooptation_username =
        env::var("COOPTATION_USERNAME").expect("Missing or invalid env var: COOPTATION_USERNAME");
    let cooptation_password =
        env::var("COOPTATION_PASSWORD").expect("Missing or invalid env var: COOPTATION_PASSWORD");
    let cooptation_token = api::cooptation::authenticate(
        cooptation_username.as_str(),
        cooptation_password.as_str(),
        cooptation_url.as_str(),
    )
    .unwrap();

    // Builds the transport mailer
    let smtp_options = service::outbox::SmtpOptions {
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
        // Get the EOS User Id
        let email = String::from(message.sender.to_string());
        let eos_user_id =
            api::eos::user_id(eos_token.to_string(), email, eos_url.as_str()).unwrap();
        // The user is not allowed
        if eos_user_id.eq(&0) {
            warn!("Invalid user: {}", message.sender);

            continue;
        }

        // POST to the Cooptation API
        let _result = api::cooptation::add(
            message,
            eos_user_id,
            cooptation_token.to_string(),
            cooptation_url.as_str(),
        )
        .unwrap();

        // SMTP send a receipt message
        let sender = format!("{}", message.sender);
        let subject = format!("Re: {}", message.subject);
        let recipient = if debug {
            let debug_recipient: String = env::var("DEBUG_RECIPIENT")
                .expect("Missing of invalid env var: DEBUG_RECIPIENT")
                .parse()
                .unwrap();
            debug_recipient.to_string()
        } else {
            debug!("Sending receipt message to: {}...", sender);

            sender.to_string()
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
        let receipt_message = service::outbox::ReceiptMessage {
            sender: env::var("SMTP_SENDER").expect("Missing or invalid env var: SMTP_SENDER"),
            recipient,
            subject,
            body: "Your request was received and will be processed shortly.\r\n\r\nBest regards\r\n\r\n--\r\nQuick Ass, your assistant".to_string(), // TODO Should be set dynamically
            in_reply_to: format!("<{}>", message.message_id),
            references,
        };
        let smtp = service::outbox::SmtpOptions {
            host: smtp_options.host.clone(),
            username: smtp_options.username.clone(),
            password: smtp_options.password.clone(),
            port: smtp_options.port.clone(),
        };
        service::outbox::send(smtp, receipt_message);
    }

    Ok(())
}
