# Handlebars-Sprig

Handlebars-Sprig is a port of the [Sprig](https://masterminds.github.io/sprig/) template functions to the Rust programming language, with support for Rust's [Handlebars library](https://crates.io/crates/handlebars).

It does not aim to be 100% compatible with Sprig. Go templates and Go idioms might not feel right in Handlebars and Rust. But Handlebars-Sprig does aim to be close enough that it feels familiar.

Handlebars-Sprig was originally developed for, and is curently used in the [Bartholomew Micro-CMS](https://github.com/fermyon/bartholomew).

## Usage

In your rust code, you need to register the handlebars helper functions:

```rust
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
    println!("Template produced: {}", hbs.render_template(tpl, &empty_context).unwrap())
}
```

The above produces the following output:

```console
$ cargo run --example basic
    Finished dev [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/examples/basic`
Template produced: 3
```
