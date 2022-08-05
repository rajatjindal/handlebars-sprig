use handlebars::{handlebars_helper, Handlebars};
use rand::seq::SliceRandom;

pub fn addhelpers(x: &mut Handlebars) {
    handlebars_helper!(contains: |sub_str: String, content: String | content.contains(&sub_str));
    handlebars_helper!(has_prefix: |prefix: String, content: String | content.starts_with(&prefix));
    handlebars_helper!(has_suffix: |suffix: String, content: String | content.ends_with(&suffix));
    handlebars_helper!(cat: |s: Vec<String>| s.join(" "));
    handlebars_helper!(replace: |target: String, replacememt: String, s: String| s.replace(&target, &replacememt));
    handlebars_helper!(shuffle: |s: String| {
        let mut bytes = s.into_bytes();
        let mut rng = rand::thread_rng();
        bytes.shuffle(&mut rng);
        let str = String::from_utf8(bytes).unwrap();
        str
    });
    handlebars_helper!(indent: |count: usize, s: String| {
        let mut data = s.clone();
        for _ in 0..count {
            data.insert(0, ' ');
        }
        data
    });

    x.register_helper("contains", Box::new(contains));
    x.register_helper("has_prefix", Box::new(has_prefix));
    x.register_helper("has_suffix", Box::new(has_suffix));
    x.register_helper("cat", Box::new(cat));
    x.register_helper("indent", Box::new(indent));
    x.register_helper("replace", Box::new(replace));
    x.register_helper("shuffle", Box::new(shuffle));
}
