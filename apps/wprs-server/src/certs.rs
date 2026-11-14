use crate::lang::{t, tf};
use std::net::IpAddr;
use tokio_rustls::rustls::crypto::aws_lc_rs;
use tokio_rustls::rustls::pki_types::pem::PemObject;
use tokio_rustls::rustls::pki_types::{CertificateDer, PrivateKeyDer};
use x509_parser::extensions::GeneralName;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Fault {
    Missing,
    Unreadable,
    NotCertificate,
    NotKey,
    Expired(i64),
    NotYet(i64),
    OtherName(String),
    OtherKey,
}

impl Fault {
    pub fn message(&self) -> String {
        match self {
            Fault::Missing => t("Write the path of this file too, or leave both empty.").into(),
            Fault::Unreadable => t("The server finds no file at this path, or cannot read it.").into(),
            Fault::NotCertificate => t("This file holds no certificate in PEM format.").into(),
            Fault::NotKey => t("This file holds no private key in PEM format.").into(),
            Fault::Expired(at) => {
                tf("This certificate expired on {}. Renew it and check again.", &[&day(*at)])
            }
            Fault::NotYet(at) => tf("This certificate is valid only from {}.", &[&day(*at)]),
            Fault::OtherName(host) => {
                tf("This certificate is not for {}, the address of this server.", &[host])
            }
            Fault::OtherKey => {
                t("This key is not the one of this certificate. Choose the one that came with it.").into()
            }
        }
    }
}

pub const MONTHS: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

fn day(at: i64) -> String {
    let Ok(when) = x509_parser::time::ASN1Time::from_timestamp(at).map(|t| t.to_datetime()) else {
        return at.to_string();
    };
    let month = MONTHS[usize::from(u8::from(when.month())) - 1];
    tf("{2} {1}, {3}", &[&when.day(), &crate::lang::word(month), &when.year()])
}

pub type Pair = (Vec<CertificateDer<'static>>, PrivateKeyDer<'static>);

#[derive(Debug, Default)]
pub struct Checked {
    pub cert: Option<Fault>,
    pub key: Option<Fault>,
    pub detail: Vec<String>,
}

pub fn load(cert: &str, key: &str) -> Result<Pair, String> {
    let chain = CertificateDer::pem_file_iter(cert)
        .and_then(|found| found.collect::<Result<Vec<_>, _>>())
        .map_err(|e| format!("{cert}: {e}"))?;
    let secret = PrivateKeyDer::from_pem_file(key).map_err(|e| format!("{key}: {e}"))?;
    Ok((chain, secret))
}

fn read_fault(path: &str, error: &str, not_pem: Fault) -> Fault {
    if std::fs::metadata(path).is_err() || error.contains("os error") { Fault::Unreadable } else { not_pem }
}

pub fn check(cert: &str, key: &str, host: &str, now: i64) -> Result<Pair, Checked> {
    let mut out = Checked::default();
    let chain = match CertificateDer::pem_file_iter(cert).and_then(|f| f.collect::<Result<Vec<_>, _>>()) {
        Ok(chain) if !chain.is_empty() => Some(chain),
        Ok(_) => {
            fail(&mut out.cert, &mut out.detail, Fault::NotCertificate, format!("{cert}: no certificate"))
        }
        Err(e) => {
            let fault = read_fault(cert, &e.to_string(), Fault::NotCertificate);
            fail(&mut out.cert, &mut out.detail, fault, format!("{cert}: {e}"))
        }
    };
    let secret = match PrivateKeyDer::from_pem_file(key) {
        Ok(secret) => Some(secret),
        Err(e) => {
            let fault = read_fault(key, &e.to_string(), Fault::NotKey);
            fail(&mut out.key, &mut out.detail, fault, format!("{key}: {e}"))
        }
    };
    if let Some(first) = chain.as_ref().and_then(|c| c.first()) {
        match x509_parser::parse_x509_certificate(first) {
            Ok((_, parsed)) => {
                let (from, until) =
                    (parsed.validity().not_before.timestamp(), parsed.validity().not_after.timestamp());
                if until < now {
                    let line =
                        format!("{cert}: certificate expired (notAfter {})", parsed.validity().not_after);
                    fail::<()>(&mut out.cert, &mut out.detail, Fault::Expired(until), line);
                } else if from > now {
                    let line = format!(
                        "{cert}: certificate not valid yet (notBefore {})",
                        parsed.validity().not_before
                    );
                    fail::<()>(&mut out.cert, &mut out.detail, Fault::NotYet(from), line);
                } else if !names_cover(&parsed, host) {
                    let line = format!("{cert}: no subject alternative name matches {host}");
                    fail::<()>(&mut out.cert, &mut out.detail, Fault::OtherName(host.into()), line);
                }
                if let Some(secret) = &secret
                    && !same_key(parsed.public_key().raw, secret)
                {
                    let line = format!("{key}: private key does not match the certificate public key");
                    fail::<()>(&mut out.key, &mut out.detail, Fault::OtherKey, line);
                }
            }
            Err(e) => drop(fail::<()>(
                &mut out.cert,
                &mut out.detail,
                Fault::NotCertificate,
                format!("{cert}: {e}"),
            )),
        }
    }
    match (chain, secret) {
        (Some(chain), Some(secret)) if out.cert.is_none() && out.key.is_none() => Ok((chain, secret)),
        _ => Err(out),
    }
}

fn fail<T>(slot: &mut Option<Fault>, detail: &mut Vec<String>, fault: Fault, line: String) -> Option<T> {
    slot.get_or_insert(fault);
    detail.push(line);
    None
}

fn names_cover(cert: &x509_parser::certificate::X509Certificate, host: &str) -> bool {
    let Ok(Some(names)) = cert.subject_alternative_name() else { return true };
    let ip: Option<IpAddr> = host.parse().ok();
    names.value.general_names.iter().any(|name| match name {
        GeneralName::DNSName(dns) => {
            let dns = dns.to_ascii_lowercase();
            let host = host.to_ascii_lowercase();
            match dns.strip_prefix("*.") {
                Some(base) => host.split_once('.').is_some_and(|(_, rest)| rest == base),
                None => dns == host,
            }
        }
        GeneralName::IPAddress(bytes) => ip.is_some_and(|ip| match ip {
            IpAddr::V4(v4) => v4.octets().as_slice() == *bytes,
            IpAddr::V6(v6) => v6.octets().as_slice() == *bytes,
        }),
        _ => false,
    })
}

fn same_key(spki: &[u8], secret: &PrivateKeyDer<'static>) -> bool {
    let provider = aws_lc_rs::default_provider();
    match provider.key_provider.load_private_key(secret.clone_key()) {
        Ok(signing) => signing.public_key().is_none_or(|public| public.as_ref() == spki),
        Err(_) => false,
    }
}
