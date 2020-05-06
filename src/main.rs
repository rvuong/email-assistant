extern crate imap;
extern crate native_tls;

use std::env;

fn main() {
    // To connect to the gmail IMAP server with this you will need to allow unsecure apps access.
    // See: https://support.google.com/accounts/answer/6010255?hl=en
    // Look at the gmail_oauth2.rs example on how to connect to a gmail server securely.
    fetch_inbox_top().unwrap();
}

fn fetch_inbox_top() -> imap::error::Result<Option<String>> {
    let imap_host = env::var("IMAP_HOST").unwrap();
    let imap_username = env::var("IMAP_USERNAME").unwrap();
    let imap_password = env::var("IMAP_PASSWORD").unwrap();
    let imap_port: u16 = env::var("IMAP_PORT").unwrap()
        .to_string()
        .parse()
        .unwrap();

    // let domain: &str = "imap.free.fr";
    let domain: &str = imap_host.as_str();

    let tls = native_tls::TlsConnector::builder().build().unwrap();

    // we pass in the domain twice to check that the server's TLS
    // certificate is valid for the domain we're connecting to.
    println!("Connecting to {:?}...", imap_host);
    let client = imap::connect(
        (domain, imap_port),
        domain,
        &tls,
    ).unwrap();
    println!("Connected: OK");

    // the client we have here is unauthenticated.
    // to do anything useful with the e-mails, we need to log in
    println!("Authenticating as {:?}...", imap_username);
    let mut imap_session = client
        .login(imap_username.as_str(), imap_password.as_str())
        .map_err(|e| e.0)?;
    println!("Authentication: OK");

    // we want to fetch the first email in the INBOX mailbox
    println!("Selecting INBOX...");
    imap_session.select("INBOX")?;
    println!("INBOX selected: OK");

    // fetch message number 1 in this mailbox, along with its RFC822 field.
    // RFC 822 dictates the format of the body of e-mails
    let messages = imap_session.fetch("1", "RFC822")?;

    let message = if let Some(m) = messages.iter().next() {
        m
    } else {
        return Ok(None);
    };

    // extract the message's body
    let body = message.body().expect("message did not have a body!");
    let body = std::str::from_utf8(body)
        .expect("message was not valid utf-8")
        .to_string();

    // be nice to the server and log out
    imap_session.logout()?;

    Ok(Some(body))
}
