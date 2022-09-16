use handlebars::{handlebars_helper, Handlebars};

pub fn addhelpers(x: &mut Handlebars) {
    handlebars_helper!(upper: |s: String| s.to_uppercase());
    handlebars_helper!(lower: |s: String| s.to_lowercase());
    handlebars_helper!(title: |s: String| {
        let mut c = s.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().chain(c).collect(),
        }
    });
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
    handlebars_helper!(abbrevboth: |l: usize, max: usize, s: String| {
        let abbr;
        if l > s.len() - 2 {
            abbr = &s[s.len() - 2..];
        } else {
            let limit = l + max;
            if limit < s.len() -2 {
                abbr = &s[l..limit];
            } else {
                abbr = &s[l..];
            }
        }
        format!("{}...", abbr)
    });
    handlebars_helper!(trim: |s:String| s.trim());
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

    handlebars_helper!(trim_suffix: |suffix: String, input: String| {
        let result = match input.strip_suffix(suffix.as_str()) {
            Some(val) => val,
            _ => input.as_str(),
        };

        result
    });

    handlebars_helper!(trim_prefix: |prefix: String, input: String|{
        let result = match input.strip_suffix(prefix.as_str()) {
            Some(val) => val,
            _ => input.as_str(),
        };

        result
    });

    handlebars_helper!(trim_all: |substr: String, input: String|{
        let firstleg = match input.strip_suffix(substr.as_str()) {
            Some(val) => val,
            _ => input.as_str(),
        };

        let result = match firstleg.strip_prefix(substr.as_str()) {
            Some(val) => val,
            _ => firstleg,
        };

        result
    });
    handlebars_helper!(repeat: |count: usize, s: String| s.repeat(count));
    handlebars_helper!(other_substr: |start: usize, end: usize, s: String| &s[start..end]);
    handlebars_helper!(nospace: |s: String| s.replace(" ", ""));
    handlebars_helper!(initials: |s: String| {
            let initial: String = s.split(" ")
            .flat_map(|s| s.chars().nth(0)).collect();
            initial.to_uppercase()
    });

    x.register_helper("upper", Box::new(upper));
    x.register_helper("lower", Box::new(lower));
    x.register_helper("title", Box::new(title));
    x.register_helper("trunc", Box::new(trunc));
    x.register_helper("abbrev", Box::new(abbrev));
    x.register_helper("abbrevboth", Box::new(abbrevboth));
    x.register_helper("plural", Box::new(plural));
    x.register_helper("trim", Box::new(trim));
    x.register_helper("join", Box::new(join));
    x.register_helper("split", Box::new(split));
    x.register_helper("splitn", Box::new(splitn));
    x.register_helper("sort_alpha", Box::new(sort_alpha));
    x.register_helper("trim_suffix", Box::new(trim_suffix));
    x.register_helper("trim_prefix", Box::new(trim_prefix));
    x.register_helper("trim_all", Box::new(trim_all));
    x.register_helper("repeat", Box::new(repeat));
    x.register_helper("substr", Box::new(other_substr));
    x.register_helper("nospace", Box::new(nospace));
    x.register_helper("initials", Box::new(initials));
}
