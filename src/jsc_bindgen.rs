extern crate url;
extern crate jscjs_sys;

use crate::jsc_api;

use std::ptr;
use std::ffi;
use std::default::Default;

pub struct VM {
    raw: jsc_api::JSContextGroupRef
}

impl VM {
    pub fn new() -> VM {
        unsafe {
            VM {
                raw: jsc_api::JSContextGroupCreate(),
            }
        }
    }
}

impl Drop for VM {
    fn drop(&mut self) {
        unsafe {
            jsc_api::JSContextGroupRelease(self.raw);
        }
    }
}

// JSC managed String.
pub struct String {
    raw: jsc_api::JSStringRef
}

impl String {
    pub fn new(s: &str) -> String {
        let cstr = ffi::CString::new(s.as_bytes()).unwrap();
        unsafe {
            String {
                raw: jsc_api::JSStringCreateWithUTF8CString(cstr.as_ptr())
            }
        }
    }

    pub fn length(&self) {
        unsafe {
            jsc_api::JSStringGetLength(self.raw);
        }
    }
}

impl Drop for String {
    fn drop(&mut self) {
        unsafe {
            jsc_api::JSStringRelease(self.raw);
        }
    }
}

pub struct Context {
    raw: jsc_api::JSGlobalContextRef
}

impl Context {
    pub fn new(vm: &VM) -> Context {
        unsafe {
            Context {
                raw: jsc_api::JSGlobalContextCreateInGroup(vm.raw, ptr::null_mut()),
            }
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            jsc_api::JSGlobalContextRelease(self.raw);
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Value {
    raw: jsc_api::JSValueRef
}

pub type JSResult<T> = Result<T, Value>;

// Value is GC-managed. So it does not implement Drop trait.
impl Value {
    pub fn with_boolean(ctx: &Context, value: bool) -> Value {
        unsafe {
            Value {
                raw: jsc_api::JSValueMakeBoolean(ctx.raw, value as u8)
            }
        }
    }

    pub fn with_number(ctx: &Context, value: f64) -> Value {
        unsafe {
            Value {
                raw: jsc_api::JSValueMakeNumber(ctx.raw, value)
            }
        }
    }

    pub fn with_string(ctx: &Context, value: &str) -> Value {
        unsafe {
            Value {
                raw: jsc_api::JSValueMakeString(ctx.raw, String::new(value).raw)
            }
        }
    }

    pub fn null(ctx: &Context) -> Value {
        unsafe {
            Value {
                raw: jsc_api::JSValueMakeNull(ctx.raw)
            }
        }
    }

    pub fn undefined(ctx: &Context) -> Value {
        unsafe {
            Value {
                raw: jsc_api::JSValueMakeUndefined(ctx.raw)
            }
        }
    }

    pub fn is_boolean(&self, ctx: &Context) -> bool {
        unsafe {
            jsc_api::JSValueIsBoolean(ctx.raw, self.raw) != 0
        }
    }

    pub fn is_null(&self, ctx: &Context) -> bool {
        unsafe {
            jsc_api::JSValueIsNull(ctx.raw, self.raw) != 0
        }
    }

    pub fn is_undefined(&self, ctx: &Context) -> bool {
        unsafe {
            jsc_api::JSValueIsUndefined(ctx.raw, self.raw) != 0
        }
    }

    pub fn is_number(&self, ctx: &Context) -> bool {
        unsafe {
            jsc_api::JSValueIsNumber(ctx.raw, self.raw) != 0
        }
    }

    pub fn is_string(&self, ctx: &Context) -> bool {
        unsafe {
            jsc_api::JSValueIsString(ctx.raw, self.raw) != 0
        }
    }

    pub fn is_object(&self, ctx: &Context) -> bool {
        unsafe {
            jsc_api::JSValueIsObject(ctx.raw, self.raw) != 0
        }
    }

    pub fn is_array(&self, ctx: &Context) -> bool {
        unsafe {
            jsc_api::JSValueIsArray(ctx.raw, self.raw) != 0
        }
    }

    pub fn is_date(&self, ctx: &Context) -> bool {
        unsafe {
            jsc_api::JSValueIsDate(ctx.raw, self.raw) != 0
        }
    }

    pub fn is_empty(&self) -> bool {
        self.raw == ptr::null()
    }

    pub fn to_number(&self, ctx: &Context) -> JSResult<f64> {
        unsafe {
            let mut exception : jsc_api::JSValueRef = ptr::null_mut();
            let result = jsc_api::JSValueToNumber(ctx.raw, self.raw, &mut exception);
            if exception == ptr::null() {
                Ok(result)
            } else {
                Err(Value { raw: exception })
            }
        }
    }

    pub fn to_boolean(&self, ctx: &Context) -> bool {
        unsafe {
            jsc_api::JSValueToBoolean(ctx.raw, self.raw) != 0
        }
    }
}

impl Default for Value {
    fn default() -> Value {
        Value { raw: ptr::null() }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Object {
    raw: jsc_api::JSObjectRef
}

impl Object {
    pub fn array(ctx: &Context, arguments: &[Value]) -> JSResult<Object> {
        unsafe {
            let mut exception : jsc_api::JSValueRef = ptr::null_mut();
            let result = jsc_api::JSObjectMakeArray(ctx.raw, arguments.len() as jsc_api::size_t, arguments.as_ptr() as *mut jsc_api::JSValueRef, &mut exception);
            if exception == ptr::null_mut() {
                Ok(Object { raw: result })
            } else {
                Err(Value { raw: exception })
            }
        }
    }

    pub fn is_constructor(&self, ctx: &Context) -> bool {
        unsafe {
            jsc_api::JSObjectIsConstructor(ctx.raw, self.raw) != 0
        }
    }
}

impl Default for Object {
    fn default() -> Object {
        Object { raw: ptr::null_mut() }
    }
}

impl Context {
    pub fn evaluate_script(&self, script: &str, receiver: &Object, url: url::Url, starting_line_number: i32) -> JSResult<Value>
    {
        let string = String::new(script);
        let source = String::new(url.as_str());
        unsafe {
            let mut exception : jsc_api::JSValueRef = ptr::null_mut();
            let result = jsc_api::JSEvaluateScript(self.raw, string.raw, receiver.raw, source.raw, starting_line_number, &mut exception);
            if exception == ptr::null_mut() {
                Ok(Value { raw: result })
            } else {
                Err(Value { raw: exception })
            }
        }
    }

    pub fn check_syntax(&self, script: &str, url: url::Url, starting_line_number: i32) -> JSResult<bool>
    {
        let string = String::new(script);
        let source = String::new(url.as_str());
        unsafe {
            let mut exception : jsc_api::JSValueRef = ptr::null_mut();
            let result = jsc_api::JSCheckScriptSyntax(self.raw, string.raw, source.raw, starting_line_number, &mut exception);
            if exception == ptr::null_mut() {
                Ok(result != 0)
            } else {
                Err(Value { raw: exception })
            }
        }
    }
}
