use handlebars::Handlebars;
use handlebars_sprig;

fn main() {
    // Get a handlebars instance
    let mut hbs = Handlebars::new();

    // THE IMPORTANT PART: Add the helpers
    handlebars_sprig::addhelpers(&mut hbs);

    // From this point, you can do whatever you want to do with the
    // handlebars instance. It will have all of the functions available
    // to the templates.
    let tpl = "{{ add 1 2 }}";

    // Example of running a template render.
    let empty_context: Vec<String> = vec![];
    println!(
        "Template produced: {}",
        hbs.render_template(tpl, &empty_context).unwrap()
    )
}
