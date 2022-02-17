use handlebars::{
    handlebars_helper, Handlebars,
};
use chrono::{DateTime, Utc};

pub fn addhelpers(x: &mut Handlebars) {
    handlebars_helper!(upper: |s: String| s.to_uppercase());
    handlebars_helper!(lower: |s: String| s.to_lowercase());
    handlebars_helper!(trunc: |l: usize, s: String| {
        let mut data = s.clone();
        data.truncate(l);
        data
    });
    handlebars_helper!(abbrev: |l: usize, s: String| {
        let mut data = s.clone();
        if l <= 3 {
            data.truncate(l);
            data
        } else if data.len() <= l {
            data
        } else {
            let l = l - 3;
            data.truncate(l);
            format!("{}...", data)
        }
    });
    handlebars_helper!(trim: |s:String| s.trim());
    handlebars_helper!(date_format: |format_string: String, date: DateTime<Utc>| {
        date.format(format_string.as_str()).to_string()
    });
    handlebars_helper!(now: |format_string: String| {
        let date = Utc::now();
        date.format(format_string.as_str()).to_string()
    });
    handlebars_helper!(plural: |count: usize, sing: String, plur: String| if count == 1 {
        sing
    } else {
        plur
    });

    handlebars_helper!(join: |delimiter: String, elements: Vec<String>|{
        elements.join(delimiter.as_str())
    });

    handlebars_helper!(split: |delimiter: String, input: String|{
        input.split(delimiter.as_str()).collect::<Vec<&str>>()
    });

    handlebars_helper!(splitn: |delimiter: String, count: usize, input: String|{
        input.splitn(count, delimiter.as_str()).collect::<Vec<&str>>()
    });

    handlebars_helper!(sort_alpha: |input: Vec<String>|{
        let mut data = input.clone();
        data.sort();
        data
    });

    x.register_helper("upper", Box::new(upper));
    x.register_helper("lower", Box::new(lower));
    x.register_helper("trunc", Box::new(trunc));
    x.register_helper("abbrev", Box::new(abbrev));
    x.register_helper("plural", Box::new(plural));
    x.register_helper("trim", Box::new(trim));
    x.register_helper("join", Box::new(join));
    x.register_helper("split", Box::new(split));
    x.register_helper("splitn", Box::new(splitn));
    x.register_helper("sort_alpha", Box::new(sort_alpha));

    // Formatting dates: https://docs.rs/chrono/latest/chrono/format/strftime/index.html#specifiers
    x.register_helper("date_format", Box::new(date_format));
    x.register_helper("now", Box::new(now));
}
