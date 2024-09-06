// Your library code goes here.
#[macro_use]
extern crate serde;
extern crate reqwest;

pub mod paths;
pub mod types;
pub mod util;

mod version {
    include!("version.rs");
}
