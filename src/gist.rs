use handlebars::{handlebars_helper, Handlebars};

pub fn addhelpers(x: &mut Handlebars) {
    handlebars_helper!(gist: |user: String, id: String| {
        let s = format!(r#"<script type="application/javascript" src="https://gist.github.com/{}/{}.js"></script>"#,  user, id);
        s
    });

    x.register_helper("gist", Box::new(gist));
}
