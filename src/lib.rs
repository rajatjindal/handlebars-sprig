use handlebars::Handlebars;

pub mod date;
pub mod gist;
pub mod list;
pub mod math;
pub mod random;
pub mod strings;
pub mod template;
pub mod tweet;

pub fn addhelpers(x: &mut Handlebars) {
    strings::addhelpers(x);
    math::addhelpers(x);
    date::addhelpers(x);
    tweet::addhelpers(x);
    gist::addhelpers(x);
    list::addhelpers(x);
    random::addhelpers(x);
    template::addhelpers(x);
}
