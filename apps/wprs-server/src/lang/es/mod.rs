mod account;
mod admin;
mod catalog;
mod chrome;
mod keeping;
mod legal;
mod mail;
mod own;
mod people;
mod policies;
mod review;
mod setup;
mod setup_done;
mod words;

type Texts = &'static [(&'static str, &'static str)];

pub const TEXTS: [Texts; 13] = [
    chrome::TEXTS,
    catalog::TEXTS,
    account::TEXTS,
    admin::TEXTS,
    review::TEXTS,
    legal::TEXTS,
    policies::TEXTS,
    own::TEXTS,
    setup::TEXTS,
    setup_done::TEXTS,
    keeping::TEXTS,
    people::TEXTS,
    mail::TEXTS,
];

pub const WORDS: Texts = words::WORDS;
