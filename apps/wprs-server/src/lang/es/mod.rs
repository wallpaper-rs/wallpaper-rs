mod account;
mod admin;
mod catalog;
mod chrome;
mod legal;
mod policies;
mod review;
mod words;

type Texts = &'static [(&'static str, &'static str)];

pub const TEXTS: [Texts; 7] = [
    chrome::TEXTS,
    catalog::TEXTS,
    account::TEXTS,
    admin::TEXTS,
    review::TEXTS,
    legal::TEXTS,
    policies::TEXTS,
];

pub const WORDS: Texts = words::WORDS;
