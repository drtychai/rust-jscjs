#![allow(unused_variables)]

extern crate jscjs as jsc;
extern crate url;
use jsc::api;

pub fn main() {
    simple();
}

fn simple() {
    unsafe {
        let vm = api::JSContextGroupCreate();
        api::JSContextGroupRelease(vm);
    }
}

fn context() {
    let vm = jsc::VM::new();
    let context = jsc::Context::new(&vm);
    let string = jsc::String::new("Hello World");
    {
        let value = jsc::Value::with_boolean(&context, false);
        assert!(value.is_boolean(&context));
    }

    {
        let value = jsc::Value::with_number(&context, 42 as f64);
        match value.to_number(&context) {
            Ok(n) => assert_eq!(n, 42 as f64),
            Err(_) => unreachable!(),
        }
    }
}

fn eval() {
    let vm = jsc::VM::new();
    let context = jsc::Context::new(&vm);
    {
        let source = url::Url::parse("https://webkit.org").unwrap();
        let object = jsc::Object::array(&context, &[]).unwrap();
        let result = context.evaluate_script("42", &object, source, 0).unwrap();
        assert!(result.is_number(&context));
    }
    {
        let source = url::Url::parse("https://webkit.org").unwrap();
        let object = jsc::Object::array(&context, &[]).unwrap();
        let result = context.evaluate_script("deadbeef", &object, source, 0);
        assert!(!result.is_ok());
    }
}

fn check_syntax() {
    let vm = jsc::VM::new();
    let context = jsc::Context::new(&vm);

    {
        let source = url::Url::parse("https://webkit.org").unwrap();
        let result = context.check_syntax("function", source, 0);
        assert!(!result.is_ok());
    }
}
