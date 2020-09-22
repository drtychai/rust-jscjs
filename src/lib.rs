#![allow(non_camel_case_types, non_snake_case)]
pub mod api;
pub mod rust;

pub use self::rust::{VM, Context, String, Value, Object};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
