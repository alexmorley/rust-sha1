#[macro_use]
extern crate neon;
extern crate crypto;

use neon::vm::{Call, JsResult};
use neon::js::JsString;

use crypto::digest::Digest;
use crypto::sha1::Sha1;

fn sha1(call: Call) -> JsResult<JsString> {
    let scope = call.scope; // Tell JS not to free objects in this scope
    let input = call.arguments.require(scope, 0)?
        .check::<JsString>()?
        .value();
    //let input = "I haven't learnt how to do that yet";

    let mut hasher = Sha1::new();
    hasher.input_str(&input);
    let hex = hasher.result_str();
    
    Ok(JsString::new(scope, &hex).unwrap())
}

register_module!(m, {
    m.export("sha1", sha1)
});
