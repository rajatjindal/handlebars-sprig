use handlebars::Handlebars;

pub mod date;
pub mod gist;
pub mod math;
pub mod strings;
pub mod tweet;

pub fn addhelpers(x: &mut Handlebars) {
    strings::addhelpers(x);
    math::addhelpers(x);
    date::addhelpers(x);
    tweet::addhelpers(x);
    gist::addhelpers(x);
}
