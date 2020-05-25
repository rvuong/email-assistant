use std::env;
use std::error::Error;

mod reader;
mod receipt;

fn main() -> Result<(), Box<dyn Error>> {
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

    let messages = reader::fetch_inbox(imap_host, imap_username, imap_password, imap_port).unwrap();
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

    // TODO Loops through IMAP messages
    for message in messages.iter() {
        let receipt_message = receipt::ReceiptMessage {
            sender: env::var("SMTP_SENDER").expect("Missing or invalid env var: SMTP_SENDER"),
            recipient: "remy.vuong@davidson.fr".to_string(), // TODO Should be set dynamically
            subject: "Re: Cooptation".to_string(),           // TODO Should be set dynamically
            body: "Your request was received and will be processed shortly.\r\n\r\nBest regards\r\n\r\n--\r\nAssistant".to_string(),   // TODO Should be set dynamically
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
