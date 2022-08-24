use handlebars::{handlebars_helper, Handlebars};
use serde::{Deserialize, Serialize};
use spin_sdk::http as spinhttp;

#[derive(Serialize, Deserialize, Debug)]
struct Publish {
    html: String,
}

pub fn addhelpers(x: &mut Handlebars) {
    handlebars_helper!(tweet: |user: String, id: String| {
        let url = format!("https://publish.twitter.com/oembed?url=https://twitter.com/{}/status/{}", user, id);
        let req = http::request::Builder::new().method("GET").uri(&url).body(None).unwrap();
        let res = spinhttp::send(req).unwrap();

        let mut html = "".to_string();
        let body = res.body().as_ref().map(|bytes| bytes.as_ref());
        let str = std::str::from_utf8(body.unwrap()).unwrap().to_string();

        if res.status().is_success() {
            let deserialized: Publish = serde_json::from_str(&str).unwrap();
            html = deserialized.html.to_string()
        } else {
            html = str
        }

        html
    });

    x.register_helper("tweet", Box::new(tweet));
}
