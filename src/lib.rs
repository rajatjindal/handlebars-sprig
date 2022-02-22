use handlebars::Handlebars;

pub mod api;
pub mod date;
pub mod math;
pub mod strings;
pub mod tweet;
pub mod types;

pub fn addhelpers(x: &mut Handlebars) {
    strings::addhelpers(x);
    math::addhelpers(x);
    date::addhelpers(x);
    tweet::addhelpers(x);
}
