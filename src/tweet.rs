use super::api::{get_products, FetchResponse};
use crate::types::Product;
use handlebars::{handlebars_helper, Handlebars};
use yew::callback::Callback;
use yew::format::Json;

//https://github.com/gohugoio/hugo/blob/02d6f5320f4202e2f403151c0aa6902f0a5e3efc/tpl/template_embedded.go
pub fn addhelpers(x: &mut Handlebars) {
    handlebars_helper!(toJSON: |s: String| {
        let callback = Callback::from(|response: FetchResponse<Vec<Product>>| {
            let (_, Json(data)) = response.into_parts();
            match data {
                Ok(product) => { eprintln!("Internal Server Error: {:?}", product);}
                Err(e) => { eprintln!("Internal Server Error: {:?}", e);},
            }
            
        });
        let y = Some(get_products(callback)).unwrap();
        "xxxxx"
    });

    x.register_helper("toJSON", Box::new(toJSON));

    let source = "hello {{ toJSON this }}";
    x.register_template_string("tweet", source).unwrap();
}
