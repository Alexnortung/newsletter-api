use phf::phf_map;
use serde_json::{json, Value};

enum Language {
    English,
}

#[derive(Clone)]
pub enum NewsletterName {
    FutureParty,
    AdobeBlog,
}

pub static NEWSLETTER_NAMES: phf::Map<&'static str, NewsletterName> = phf_map! {
    "futureparty" => NewsletterName::FutureParty,
    "adobeblog" => NewsletterName::AdobeBlog,
};

struct Newsletter {
    name: NewsletterName,
    language: Option<Language>,
}

pub fn string_to_newsletter_name(name: &str) -> Option<&NewsletterName> {
    NEWSLETTER_NAMES.get(name)
}

pub async fn signup_newsletter(
    email: &str,
    newsletter_name_string: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let newsletter_name_option = string_to_newsletter_name(newsletter_name_string);

    if newsletter_name_option.is_none() {
        return Err(format!("Unknown newsletter: {}", newsletter_name_string).into());
    }

    let newsletter_name = newsletter_name_option.unwrap();

    let builder = match newsletter_name {
        NewsletterName::FutureParty => client.post("https://front.optimonk.com/public/186873/creative/641f4d44b3e878002490f77d/conversionExtended").form(&[("visitor[email]", email)]),
        // "thehustle" => client.post("https://forms.hsforms.com/submissions/v3/public/submit/formsnext/multipart/20627419/f93a9319-4b52-4b83-a6c6-22d2427ae71d").form(&[("email", email)]), // uses recapcha
        NewsletterName::AdobeBlog => client.post("https://www.adobe.com/api2/subscribe_v1").json(&json!({
            "consent_notice": "\n\n            <p style=\"text-align: center;\">The&nbsp;<a href=\"https://www.adobe.com/privacy/policy.html#info-share\">Adobe family of companies</a>&nbsp;may keep me informed with&nbsp;<a href=\"https://www.adobe.com/privacy/marketing.html#mktg-email\">personalized </a>emails about Adobe Blog Newsletter. See our&nbsp;<a href=\"https://www.adobe.com/privacy/policy.html\">Privacy Policy</a>&nbsp;for more details or to opt out at any time.</p>\n\n        ",
            "current_url": "https://www.adobe.com/subscription/adobeblognewsletter.html",
            "email": email,
            "sname": "adbeblognewsletter",
        })),
    };

    let response = builder.send().await?;

    let status = response.status();
    if !status.is_success() {
        return Err(format!(
            "Could not signup for newsletter: {}; with status code: {}",
            newsletter_name_string,
            status.as_u16()
        )
        .into());
    }

    Ok(())
}
