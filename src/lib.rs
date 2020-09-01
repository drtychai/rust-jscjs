#![allow(non_camel_case_types, non_snake_case)]

pub mod jsc_api;
pub mod jsc_bindgen;

pub use self::jsc_bindgen::{VM, Context, String, Value, Object};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
