mod account;
mod admin;
mod catalog;
mod chrome;
mod legal;
mod own;
mod policies;
mod review;
mod words;

type Texts = &'static [(&'static str, &'static str)];

pub const TEXTS: [Texts; 8] = [
    chrome::TEXTS,
    catalog::TEXTS,
    account::TEXTS,
    admin::TEXTS,
    review::TEXTS,
    legal::TEXTS,
    policies::TEXTS,
    own::TEXTS,
];

pub const WORDS: Texts = words::WORDS;
