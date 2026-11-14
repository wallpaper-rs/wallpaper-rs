use super::{Security, Setup};
use crate::lang::t;
use lettre::message::header::ContentType;
use lettre::message::{Mailbox, Message};
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::client::{Tls, TlsParameters};
use lettre::transport::smtp::extension::ClientId;
use lettre::{Address, AsyncSmtpTransport, AsyncTransport, Tokio1Executor};
use std::time::Duration;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Why {
    Password,
    Reach,
    Secure,
    Address,
    Other,
}

impl Why {
    const ALL: [Why; 5] = [Why::Password, Why::Reach, Why::Secure, Why::Address, Why::Other];

    pub fn value(self) -> &'static str {
        match self {
            Why::Password => "password",
            Why::Reach => "reach",
            Why::Secure => "secure",
            Why::Address => "address",
            Why::Other => "other",
        }
    }

    pub fn parse(value: &str) -> Why {
        Why::ALL.into_iter().find(|w| w.value() == value).unwrap_or(Why::Other)
    }

    pub fn said(self) -> &'static str {
        match self {
            Why::Password => t("the mail server refused the password"),
            Why::Reach => t("the mail server could not be reached"),
            Why::Secure => t("the secure connection to the mail server failed"),
            Why::Address => t("the mail server refused the address"),
            Why::Other => t("the mail server answered with an error"),
        }
    }
}

pub struct Failure {
    pub why: Why,
    pub raw: String,
}

pub fn valid_from(from: &str) -> bool {
    from.parse::<Mailbox>().is_ok()
}

pub fn valid_to(to: &str) -> bool {
    to.parse::<Address>().is_ok()
}

fn sorted(error: &lettre::transport::smtp::Error) -> Why {
    let code = error.status().map(|c| c.to_string());
    match code.as_deref() {
        Some("530" | "534" | "535" | "538") => Why::Password,
        Some("501" | "550" | "551" | "553" | "554") => Why::Address,
        Some(_) => Why::Other,
        None if error.is_tls() => Why::Secure,
        None if error.is_client() || error.is_response() => Why::Other,
        None => Why::Reach,
    }
}

fn message(setup: &Setup, subject: &str, body: &str) -> Result<Message, Failure> {
    let bad = |e: &dyn std::fmt::Display| Failure { why: Why::Address, raw: format!("message: {e}") };
    let from: Mailbox = setup.from.parse().map_err(|e| bad(&e))?;
    let to: Mailbox = setup.to.parse().map_err(|e| bad(&e))?;
    Message::builder()
        .from(from)
        .to(to)
        .subject(subject)
        .header(ContentType::TEXT_PLAIN)
        .body(body.to_owned())
        .map_err(|e| bad(&e))
}

pub async fn send(
    setup: &Setup,
    secret: &str,
    hello: &str,
    subject: &str,
    body: &str,
) -> Result<(), Failure> {
    let mail = message(setup, subject, body)?;
    let place = format!("smtp {}:{}", setup.host, setup.port);
    let tls = |params: TlsParameters| match setup.security {
        Security::StartTls => Tls::Required(params),
        Security::Tls => Tls::Wrapper(params),
        Security::Plain => Tls::None,
    };
    let tls = match setup.security {
        Security::Plain => Tls::None,
        _ => tls(TlsParameters::new(setup.host.clone())
            .map_err(|e| Failure { why: Why::Secure, raw: format!("{place}: {e}") })?),
    };
    let mut builder = AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&setup.host)
        .port(setup.port)
        .tls(tls)
        .hello_name(ClientId::Domain(hello.to_owned()))
        .timeout(Some(Duration::from_secs(30)));
    if !setup.user.is_empty() {
        builder = builder.credentials(Credentials::new(setup.user.clone(), secret.to_owned()));
    }
    match builder.build().send(mail).await {
        Ok(_) => Ok(()),
        Err(e) => Err(Failure { why: sorted(&e), raw: format!("{place}: {e}") }),
    }
}
