use js_sys::JsString;
use node_sys::{fs, os, path};

pub fn tmpdir() -> JsString {
    let prefix = &path::join({
        let mut val = vec![];
        val.push(os::tmpdir().into());
        val.push("node-sys".into());
        val.into_boxed_slice()
    });
    let options = Default::default();
    fs::mkdtemp_sync(prefix, options)
}

pub fn tmpfile(filename: &str) -> JsString {
    path::join({
        let mut val = vec![];
        val.push(tmpdir().into());
        val.push(filename.into());
        val.into_boxed_slice()
    })
}
