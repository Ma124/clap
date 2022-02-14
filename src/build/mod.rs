#[macro_use]
mod macros;

mod app;
mod app_settings;
mod arg;
mod arg_group;
mod arg_predicate;
mod arg_settings;
mod possible_value;
mod usage_parser;
mod value_hint;

#[cfg(feature = "regex")]
mod regex;

#[cfg(debug_assertions)]
mod debug_asserts;

#[cfg(test)]
mod tests;

pub use app::App;
pub use app_settings::{AppFlags, AppSettings};
pub use arg::Arg;
pub use arg_group::ArgGroup;
pub use arg_settings::{ArgFlags, ArgSettings};
pub use possible_value::PossibleValue;
pub use value_hint::ValueHint;

#[cfg(feature = "regex")]
pub use self::regex::RegexRef;

pub(crate) use arg::display_arg_val;
pub(crate) use arg_predicate::ArgPredicate;
