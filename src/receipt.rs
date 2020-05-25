extern crate lettre;
extern crate native_tls;
extern crate uuid;

use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::smtp::ConnectionReuseParameters;
use lettre::{
    ClientSecurity, ClientTlsParameters, EmailAddress, Envelope, SendableEmail, SmtpClient,
    Transport,
};
use uuid::Uuid;

use self::native_tls::{Protocol, TlsConnector};

pub struct SmtpOptions {
    pub host: String,
    pub username: String,
    pub password: String,
    pub port: u16,
}

pub struct ReceiptMessage {
    pub sender: String,
    pub recipient: String,
    pub subject: String,
    pub body: String,
}

pub fn send(smtp_options: SmtpOptions, receipt_message: ReceiptMessage) {
    let mut tls_builder = TlsConnector::builder();
    tls_builder.min_protocol_version(Some(Protocol::Tlsv10));
    let tls_parameters =
        ClientTlsParameters::new(smtp_options.host.to_string(), tls_builder.build().unwrap());
    let mut mailer = SmtpClient::new(
        (smtp_options.host.as_str(), smtp_options.port),
        ClientSecurity::Wrapper(tls_parameters),
    )
    .unwrap()
    .smtp_utf8(true)
    .authentication_mechanism(Mechanism::Login)
    .credentials(Credentials::new(
        smtp_options.username,
        smtp_options.password,
    ))
    .connection_reuse(ConnectionReuseParameters::ReuseUnlimited)
    .transport();

    let email = SendableEmail::new(
        Envelope::new(
            Some(EmailAddress::new(receipt_message.sender).unwrap()),
            vec![EmailAddress::new(receipt_message.recipient).unwrap()],
        )
        .unwrap(),
        Uuid::new_v4().to_string(),
        format!(
            "Subject: {subject}\r\n\r\n{body}\r\n\r\n",
            subject = receipt_message.subject,
            body = receipt_message.body
        )
        .into_bytes(),
    );

    let result = mailer.send(email);
    assert!(result.is_ok());

    // Explicitly close the SMTP transaction as we enabled connection reuse
    mailer.close();
}