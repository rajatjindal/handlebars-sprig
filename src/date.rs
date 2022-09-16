use chrono::{DateTime, Utc};
use handlebars::{handlebars_helper, Handlebars};

pub fn addhelpers(x: &mut Handlebars) {
    handlebars_helper!(date_format: |format_string: String, date: DateTime<Utc>| {
        date.format(format_string.as_str()).to_string()
    });
    handlebars_helper!(now: |format_string: String| {
        let date = Utc::now();
        date.format(format_string.as_str()).to_string()
    });

    // Formatting dates: https://docs.rs/chrono/latest/chrono/format/strftime/index.html#specifiers
    x.register_helper("date_format", Box::new(date_format));
    x.register_helper("now", Box::new(now));
}
