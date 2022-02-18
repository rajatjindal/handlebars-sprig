use handlebars::{handlebars_helper, Handlebars};
use rand::Rng;

pub fn addhelpers(x: &mut Handlebars) {
    handlebars_helper!(add: |a: isize, b: isize| a + b);
    handlebars_helper!(sub: |a: isize, b: isize| a - b);
    handlebars_helper!(mul: |a: isize, b: isize| a * b);
    handlebars_helper!(div: |a: isize, b: isize| a / b);
    handlebars_helper!(modulus: |a: isize, b: isize| a % b);
    handlebars_helper!(max: |a: isize, b: isize| if a > b { a } else { b });
    handlebars_helper!(min: |a: isize, b: isize| if a < b { a } else { b });
    handlebars_helper!(floor: |a: f64| a.floor());
    handlebars_helper!(ceil: |a: f64| a.ceil());
    handlebars_helper!(round: |a: f64| a.round());
    handlebars_helper!(randInt:  | | rand::thread_rng().gen::<usize>());

    x.register_helper("add", Box::new(add));
    x.register_helper("sub", Box::new(sub));
    x.register_helper("mul", Box::new(mul));
    x.register_helper("div", Box::new(div));
    x.register_helper("mod", Box::new(modulus));
    x.register_helper("max", Box::new(max));
    x.register_helper("floor", Box::new(floor));
    x.register_helper("ceil", Box::new(ceil));
    x.register_helper("round", Box::new(round));
    x.register_helper("randInt", Box::new(randInt));
}
