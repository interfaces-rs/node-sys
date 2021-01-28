use js_sys::JsString;
use node_sys::{fs, os, path};

pub fn tmpdir() -> JsString {
    let prefix = &path::join({
        let val = vec![os::tmpdir().into(), "node-sys".into()];
        val.into_boxed_slice()
    });
    let options = Default::default();
    fs::mkdtemp_sync(prefix, options)
}

pub fn tmpfile(filename: &str) -> JsString {
    path::join({
        let val = vec![tmpdir().into(), filename.into()];
        val.into_boxed_slice()
    })
}
