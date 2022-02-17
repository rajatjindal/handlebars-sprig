use handlebars::Handlebars;

pub mod strings;
pub mod math;
pub mod date;

pub fn addhelpers(x: &mut Handlebars) {
    strings::addhelpers(x);
    math::addhelpers(x);
    date::addhelpers(x)
}