use wprs_core::message::{Message, said};

pub const KNOWN: [&str; 10] = [
    "CC0-1.0",
    "CC-BY-4.0",
    "CC-BY-SA-4.0",
    "CC-BY-NC-4.0",
    "CC-BY-NC-SA-4.0",
    "CC-BY-ND-4.0",
    "CC-BY-NC-ND-4.0",
    "MIT",
    "Apache-2.0",
    "GPL-3.0-or-later",
];

pub fn describe(expression: &str) -> Message {
    match expression.trim() {
        "CC0-1.0" => said(
            "format.license_cc0",
            "Public domain: anyone may use, change and share it with no conditions",
            &[],
        ),
        "CC-BY-4.0" => said(
            "format.license_by",
            "Anyone may use, change and share it, even commercially, if they credit you",
            &[],
        ),
        "CC-BY-SA-4.0" => said(
            "format.license_by_sa",
            "Like CC-BY, and whatever is made from it must be shared under the same license",
            &[],
        ),
        "CC-BY-NC-4.0" => said(
            "format.license_by_nc",
            "Anyone may change and share it if they credit you, but not for commercial use",
            &[],
        ),
        "CC-BY-NC-SA-4.0" => said(
            "format.license_by_nc_sa",
            "Credit you, no commercial use, and whatever is made from it uses the same license",
            &[],
        ),
        "CC-BY-ND-4.0" => said(
            "format.license_by_nd",
            "Anyone may share it unchanged if they credit you; changed versions are not allowed",
            &[],
        ),
        "CC-BY-NC-ND-4.0" => said(
            "format.license_by_nc_nd",
            "Anyone may share it unchanged, for free, if they credit you; changed versions are not allowed",
            &[],
        ),
        "MIT" => said(
            "format.license_mit",
            "Anyone may use, change and share it, even commercially, if they keep your copyright notice",
            &[],
        ),
        "Apache-2.0" => said(
            "format.license_apache",
            "Like MIT, and it also grants patent rights and asks to note what was changed",
            &[],
        ),
        "GPL-3.0-or-later" => said(
            "format.license_gpl",
            "Anyone may use, change and share it, but every shared version must stay free under the same terms",
            &[],
        ),
        text if text.starts_with("LicenseRef-") => said(
            "format.license_own",
            "Your own terms: the package must explain them where it is published",
            &[],
        ),
        _ => said(
            "format.license_other",
            "Not one of the usual licenses: read its terms before using or changing the package",
            &[],
        ),
    }
}

pub fn condition(name: &str) -> Message {
    match name {
        "attribution" => said("format.condition_by", "credit the original author", &[]),
        "share-alike" => said("format.condition_sa", "share your version under the same license", &[]),
        "non-commercial" => said("format.condition_nc", "do not use it commercially", &[]),
        "copyleft" => said("format.condition_copyleft", "keep it free under the same terms", &[]),
        _ => said("format.condition_own", "follow the terms the license states", &[]),
    }
}
