extern crate hyper;
extern crate google_translate3 as translate3;
extern crate tokio;

use translate3::api::Glossary;
use translate3::{Translate, oauth2 ,hyper_rustls, Result, Error};
use std::default::Default;

#[tokio::main]
async fn main() {
    let blocking_task = tokio::task::spawn(do_async_work());
    blocking_task.await.unwrap();
}

async fn do_async_work() {
    println!("Hello, world!");
    let secret: oauth2::ApplicationSecret = Default::default();
    let client = hyper::Client::new();
    let auth = oauth2::InstalledFlowAuthenticator::builder(
        secret,
        oauth2::InstalledFlowReturnMethod::HTTPRedirect,
    ).build().await.unwrap();
    let mut hub = Translate::new(client, auth);

    // Translate from English to Chinese Simplified
    let from = "en";
    let to = "zh-CN"; 
    let mut req = Glossary::default();

    let result = hub.projects().locations_glossaries_patch(req, "name")
             .update_mask(Default::default())
             .doit().await;

    match result {
        Err(e) => match e {
            Error::HttpError(_) |
            Error::Io(_) |
            Error::MissingAPIKey |
            Error::Cancelled |
            Error::UploadSizeLimitExceeded(_, _) |
            Error::Failure(_) |
            Error::BadRequest(_) |
            Error::MissingToken(_) |
            Error::FieldClash(_) |
            Error::JsonDecodeError(_, _) 
            => println!("{}", e)
        },
        Ok(res) => println!("Success: {:?}", res),
    } 
}