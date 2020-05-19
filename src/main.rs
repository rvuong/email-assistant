extern crate imap;
extern crate native_tls;

use native_tls::TlsConnector;
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let imap_host = env::var("IMAP_HOST")
        .expect("Missing or invalid env var: IMAP_HOST");
    let imap_username = env::var("IMAP_USERNAME")
        .expect("Missing or invalid env var: IMAP_USERNAME");
    let imap_password = env::var("IMAP_PASSWORD")
        .expect("Missing or invalid env var: IMAP_PASSWORD");
    let imap_port: u16 = env::var("IMAP_PORT")
        .expect("Missing or invalid env var: IMAP_PORT")
        .to_string()
        .parse()
        .unwrap();

    if let Some(_email) = fetch_inbox_top(imap_host, imap_username, imap_password, imap_port)? {
        eprintln!("OK :)");
    }

    Ok(())
}

fn fetch_inbox_top(
    host: String,
    username: String,
    password: String,
    port: u16
) -> Result<Option<String>, Box<dyn Error>> {
    let domain: &str = host.as_str();

    let tls = TlsConnector::builder().build().unwrap();

    // we pass in the domain twice to check that the server's TLS
    // certificate is valid for the domain we're connecting to.
    let client = imap::connect_starttls(
        (domain, port),
        domain,
        &tls,
    ).unwrap();
    eprintln!("[ OK ] Connected to {:?}:{:?}.", host, port);

    // the client we have here is unauthenticated.
    // to do anything useful with the e-mails, we need to log in
    let mut imap_session = client
        .login(username.as_str(), password.as_str())
        .map_err(|e| e.0)?;
    eprintln!("[ OK ] Authenticated as {:?}.", username.as_str());

    // we want to fetch the first email in the INBOX mailbox
    imap_session.select("INBOX")?;
    eprintln!("[ OK ] Inbox {:?} selected.", "INBOX");

    // Search & filter the messages in the inbox
    let messages = imap_session.search("UNSEEN")?;
    eprintln!("[ OK ] Unseen messages:\n{:#?}", messages);

    let message = if let Some(m) = messages.iter().next() {
        m
    } else {
        return Ok(None);
    };
    eprintln!("{:#?}", message);

    // let fetched = imap_session.fetch("62", "ALL");
    // eprintln!("{:#?}", fetched);

    /*
    let body = message.body().expect("message did not have a body!");
    let body = std::str::from_utf8(body)
        .expect("message was not valid utf-8")
        .to_string();
    */

    // be nice to the server and log out
    imap_session.logout()?;

    // Ok(Some(body))

    Ok(None)
}
