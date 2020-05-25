extern crate native_tls;

use std::error::Error;

use imap::types::Fetch;
use native_tls::TlsConnector;

pub struct Message {
    pub sender: String,
    pub subject: String,
    pub body: String,
    pub attachment: String,
}

pub fn fetch_inbox(
    host: String,
    username: String,
    password: String,
    port: u16,
) -> Result<Vec<self::Message>, Box<dyn Error>> {
    let mut v: Vec<self::Message> = Vec::new();

    let tls = TlsConnector::builder().build().unwrap();

    // we pass in the domain twice to check that the server's TLS
    // certificate is valid for the domain we're connecting to.
    let client = imap::connect_starttls((host.as_str(), port), host.as_str(), &tls).unwrap();
    eprintln!("[ OK ] Connected to {:?}:{:?}.", host, port);

    // the client we have here is unauthenticated.
    // to do anything useful with the e-mails, we need to log in
    let mut imap_session = client
        .login(username.as_str(), password.as_str())
        .map_err(|e| e.0)?;

    // we want to fetch the first email in the INBOX mailbox
    imap_session.select("INBOX")?;

    // Search & filter the messages in the inbox
    let message_ids = imap_session.search("UNSEEN")?;
    if message_ids.len().eq(&0) {
        return Ok(v);
    }

    for message_id in &message_ids {
        let messages = imap_session.fetch(format!("{}", message_id), "RFC822")?;
        let message = if let Some(m) = messages.iter().next() {
            m
        } else {
            return Ok(v);
        };

        let data = self::get_message(message);
        v.push(data);

        // Temporary set messages as unread again
        // imap_session.store(format!("{}", message_id), "-FLAGS (\\Seen)")?;
    }

    // be nice to the server and log out
    imap_session.logout()?;

    Ok(v)
}

fn get_message(_message: &Fetch) -> self::Message {
    let m = self::Message {
        sender: "remy.vuong@davidson.fr".to_string(),
        subject: "cooptation".to_string(),
        body: "TODO".to_string(),
        attachment: "TODO".to_string(),
    };

    // TODO Extract the sender's email address
    // TODO Split the header into several rows
    // TODO Extract the email subject
    // TODO Extract the email body text
    // TODO Extract the 1st attachment (CV)

    return m;
}
