mod account;
mod admin;
mod catalog;
mod chrome;
mod legal;
mod own;
mod policies;
mod review;
mod setup;
mod setup_done;
mod words;

type Texts = &'static [(&'static str, &'static str)];

pub const TEXTS: [Texts; 10] = [
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
];

pub const WORDS: Texts = words::WORDS;
