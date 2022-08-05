use handlebars::{handlebars_helper, Handlebars};
use rand::distributions::Alphanumeric;
use rand::Rng;

pub fn addhelpers(x: &mut Handlebars) {
    handlebars_helper!(rand_numeric: | | rand::thread_rng().gen_range(0..10));
    handlebars_helper!(rand_alpha: | |  {
        let val: u8 =  65 + rand::thread_rng().gen_range(0..26) + (rand::thread_rng().gen_range(0..2) * 32);
        let mut s = String::new();
        s.insert(0, val as char);
        s
    });
    handlebars_helper!(rand_alphanumeric: | | {
        let rand_string: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(1)
        .map(char::from)
        .collect();
        rand_string
    });

    x.register_helper("rand_numeric", Box::new(rand_numeric));
    x.register_helper("rand_alpha", Box::new(rand_alpha));
    x.register_helper("rand_alphanumeric", Box::new(rand_alphanumeric));
}
