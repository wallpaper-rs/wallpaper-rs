use crate::told::{fill, told, tr};
use crate::usage;
use anyhow::{Context, Result, bail, ensure};
use percent_encoding::percent_decode_str;
use std::collections::HashMap;
use std::io::{self, BufRead, IsTerminal};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use wprs_client::{Client, explain};
use wprs_core::dbus::uri_scheme;

pub fn path_arg(text: &str) -> Result<String> {
    let local = || usage(fill(told("%1 is not a local file"), &[&text]));
    match uri_scheme(text).filter(|_| !Path::new(text).exists()) {
        Some(scheme) if scheme.eq_ignore_ascii_case("file") => {
            let rest = &text[scheme.len() + 1..];
            let path = rest.strip_prefix("//").map_or(rest, |r| r.strip_prefix("localhost").unwrap_or(r));
            let path = percent_decode_str(path).decode_utf8().map_err(|_| local())?;
            path.starts_with('/').then(|| path.into_owned()).ok_or_else(local)
        }
        Some(_) => Err(local()),
        None => std::path::absolute(text)?.into_os_string().into_string().map_err(|_| local()),
    }
}

pub fn item_ref(text: &str) -> Result<String> {
    if text.contains('/') || Path::new(text).exists() { path_arg(text) } else { Ok(text.into()) }
}

pub async fn resolve(client: &Client, items: &[String]) -> Result<Vec<String>> {
    let refs = items.iter().map(|item| item_ref(item)).collect::<Result<Vec<_>>>()?;
    client.library.resolve(&strs(&refs)).await.map_err(explain)
}

pub async fn one(client: &Client, item: &str) -> Result<String> {
    resolve(client, &[item.into()]).await?.pop().context(tr(told("wprsd resolved no item")))
}

pub fn strs(items: &[String]) -> Vec<&str> {
    items.iter().map(String::as_str).collect()
}

pub fn pairs(items: &[String]) -> Result<HashMap<&str, &str>> {
    items
        .iter()
        .map(|item| item.split_once('=').ok_or_else(|| usage(fill(told("%1 is not KEY=VALUE"), &[&item]))))
        .collect()
}

pub fn secret(prompt: &str, stdin: bool) -> Result<String> {
    if io::stdin().is_terminal() {
        return Ok(if stdin { rpassword::read_password()? } else { rpassword::prompt_password(prompt)? });
    }
    let mut line = String::new();
    io::stdin().lock().read_line(&mut line)?;
    line.truncate(line.trim_end_matches(['\r', '\n']).len());
    Ok(line)
}

pub const PASSWORD_VARIABLE: &str = "WPRS_KEY_PASSWORD";

#[derive(Debug, PartialEq)]
pub enum Password {
    Given(String),
    Ask,
    Missing,
}

pub fn password_source(variable: Option<String>, terminal: bool) -> Password {
    match variable.filter(|v| !v.is_empty()) {
        Some(given) => Password::Given(given),
        None if terminal => Password::Ask,
        None => Password::Missing,
    }
}

pub fn password(twice: bool) -> Result<String> {
    let source = password_source(std::env::var(PASSWORD_VARIABLE).ok(), io::stdin().is_terminal());
    match source {
        Password::Given(given) => Ok(given),
        Password::Missing => {
            bail!(
                "{}",
                fill(
                    told(
                        "the key has a password and there is no terminal to ask for it: set the environment variable %1 to the password"
                    ),
                    &[&PASSWORD_VARIABLE]
                )
            )
        }
        Password::Ask => {
            let password = rpassword::prompt_password(tr(told("Key password: ")))?;
            ensure!(
                !twice || rpassword::prompt_password(tr(told("Repeat the password: ")))? == password,
                "{}",
                tr(told("the passwords differ"))
            );
            Ok(password)
        }
    }
}

pub async fn confirm(question: &str) -> Result<bool> {
    if !io::stdin().is_terminal() {
        return Ok(false);
    }
    eprint!("{question} [y/N] ");
    let line = typed().await?;
    Ok(matches!(line.trim().to_ascii_lowercase().as_str(), "y" | "yes"))
}

pub async fn typed() -> Result<String> {
    let (tx, rx) = tokio::sync::oneshot::channel();
    std::thread::spawn(move || {
        let mut line = String::new();
        tx.send(io::stdin().lock().read_line(&mut line).map(|_| line))
    });
    tokio::select! {
        line = rx => Ok(line.context(tr(told("the terminal closed")))??),
        _ = tokio::signal::ctrl_c() => {
            eprintln!();
            bail!("{}", tr(told("stopped without an answer")))
        }
    }
}

pub fn now() -> Result<i64> {
    Ok(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs().try_into()?)
}

#[cfg(test)]
mod tests {
    use super::{Password, password_source};

    #[test]
    fn the_password_comes_from_the_variable_then_the_terminal_and_is_otherwise_missing() {
        let given = |v: &str| Some(v.to_owned());
        assert_eq!(password_source(given("x"), false), Password::Given("x".into()));
        assert_eq!(password_source(given("x"), true), Password::Given("x".into()));
        assert_eq!(password_source(given(""), true), Password::Ask);
        assert_eq!(password_source(None, true), Password::Ask);
        assert_eq!(password_source(None, false), Password::Missing);
    }
}
