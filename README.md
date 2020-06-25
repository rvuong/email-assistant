# Email Assistant

*Email Assistant* automatically triggers actions, like API requests, when you send it emails.

## TL;DR

It should be able to read an Inbox via IMAP, and to request a bunch of various APIs. *Email Assistant* will also send an email response in order to keep the user well informed.

Watch a simple use-case [footage](https://www.loom.com/share/e4a876ec15b34b03bff413d934fdd418):

- The user sends and email
  - to my own email address, for testing purpose
  - the subject contains the word "cooptation"
  - the email is text/plain
  - the users adds an attachment
- *Email Assistant* is run "by hand"
- The cooptation resource was created and can be read through *Postman*
  - The attachment was uploaded to the engine server AND to the API server
- The user receives a generic email Reply

### Email Assistant does

- **Identify the user** on the system of the company, given her/his email address
    - and rejects her/his request if the user was not identified
- **Create** a new cooptation resource
  - when the subject of the incoming email contains "cooptation"
  - with partially mocked data
- **Send an email** response as a notification

### Email Assistant does not do (yet)

- NLP actions, including:
  - Extract accurate data from the email body/the attachment file: firstname, lastname, phone number, and email address of the co-opted person.
  - Convert a HTML message to text/plain (which is requested by the API)
  - Extract the branch ids that the person should be co-opted to.
  - Identify the accurate requested action ("cooptation", ...) from the email subject/body.
- Multi-messages conversation
- Multi-language support (various email notifications)
- Async processing
- Run in background (as a daemon). The process should be run by cron for now.

## Software architecture

*Email Assistant* is composed in several modules:

- [x] an IMAP client, for reading the Inbox
- [ ] a Natural Language Processing engine (NLP engine), in order to recognize the user request and act accordingly
- [x] several API connectors, to execute the appropriate actions
- [x] a SMTP client, in order to send email notifications
- [ ] an error management system, including logs and notifications

## Dev

Get the [dev environment](./README_dev.md) ready.

Done!
