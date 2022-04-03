use handlebars::{handlebars_helper, Handlebars};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Publish {
    html: String,
}

pub fn addhelpers(x: &mut Handlebars) {
    handlebars_helper!(tweet: |user: String, id: String| {
        let url = format!("https://publish.twitter.com/oembed?url=https://twitter.com/{}/status/{}", user, id);
        let req = http::request::Builder::new().uri(&url).body(None).unwrap();
        let mut res = wasi_experimental_http::request(req).expect("cannot make get request");

        let mut html = "".to_string();
        if res.status_code == 200 {
            let str = std::str::from_utf8(&res.body_read_all().unwrap()).unwrap().to_string();
            let deserialized: Publish = serde_json::from_str(&str).unwrap();
            html = deserialized.html.to_string()
        }

        html
    });

    x.register_helper("tweet", Box::new(tweet));
}
