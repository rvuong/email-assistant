# Email Assistant

## TL;DR

The purpose of *Email Assistant* is to automatically trigger various actions, when specific emails are received.

It should be able to read an Inbox via IMAP, and request a bunch of various APIs. `Email Assistant` will also send an email response in order to keep the user well informed.

`Email Assistant` is composed in several modules:

- [x] an IMAP client, for reading the Inbox
- [ ] a Natural language processing engine, in order to recognize the user request and act accordingly
- [ ] an API gateway, to execute the appropriate actions
- [x] a SMTP client, in order to send email notifications
- [ ] an error management system, including logs and notifications

## Dev

Howto get the [dev environment](./README_dev.md) ready.

Done!
