use node_sys::process;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn abort() {
    // if let Err(_err) = process.abort() {
    //     // #[should_panic]
    // }
    // FIXME
}

#[wasm_bindgen_test]
fn arch() {
    process.arch();
}

#[wasm_bindgen_test]
fn argv() {
    process.argv();
}

#[wasm_bindgen_test]
fn argv0() {
    process.argv0();
}

// FIXME
#[wasm_bindgen_test]
fn connected() {
    // process.connected();
}

#[wasm_bindgen_test]
fn cpu_usage() {
    let previous_value = Default::default();
    process.cpu_usage(previous_value);
}

#[wasm_bindgen_test]
fn cwd() {
    process.cwd();
}

#[wasm_bindgen_test]
fn debug_port() {
    process.debug_port();
}

// FIXME
#[wasm_bindgen_test]
fn disconnect() {
    // process.disconnect();
}

#[wasm_bindgen_test]
fn domain() {
    use wasm_bindgen::{prelude::*, JsCast};
    match process.domain() {
        None => {},
        Some(domain) => {
            let clo = Closure::wrap(Box::new(|x: JsValue| x) as Box<dyn Fn(JsValue) -> JsValue>);
            let fun = clo.as_ref().unchecked_ref();
            domain.add_with_emitter(&process);
            domain.bind(&fun);
            domain.intercept(&fun);
            domain.remove_with_emitter(&process);
            domain.run(&fun, Box::new([0.into()]));
            clo.forget();
        },
    }
}

#[wasm_bindgen_test]
fn emit_warning() {
    let warning = &<&str as Default>::default().into();
    let name = Default::default();
    let ctor = Default::default();
    process.emit_warning(warning, name, ctor);
}

#[wasm_bindgen_test]
fn env() {
    process.env();
}

#[wasm_bindgen_test]
fn exec_argv() {
    process.exec_argv();
}

#[wasm_bindgen_test]
fn exec_path() {
    process.exec_path();
}

#[wasm_bindgen_test]
fn exit() {
    // process.exit();
}

#[wasm_bindgen_test]
fn exit_code() {
    process.exit_code();
}

#[wasm_bindgen_test]
fn get_egid() {
    process.get_egid();
}

#[wasm_bindgen_test]
fn get_euid() {
    process.get_euid();
}

#[wasm_bindgen_test]
fn get_gid() {
    process.get_gid();
}

#[wasm_bindgen_test]
fn get_groups() {
    process.get_groups();
}

#[wasm_bindgen_test]
fn get_uid() {
    process.get_gid();
}

#[wasm_bindgen_test]
fn hrtime() {
    process.hr_time();
}

#[wasm_bindgen_test]
fn kill() {
    let pid = 999_999;
    if let Err(_err) = process.kill(pid) {}
}

#[wasm_bindgen_test]
fn kill_with_signal_name() {
    let pid = 999_999;
    let signal_name = &<&str as Default>::default().into();
    if let Err(_err) = process.kill_with_signal_name(pid, signal_name) {}
}

#[wasm_bindgen_test]
fn kill_with_signal_id() {
    let pid = 999_999;
    let signal_id = Default::default();
    if let Err(_err) = process.kill_with_signal_id(pid, signal_id) {}
}

#[wasm_bindgen_test]
fn main_module() {
    match process.main_module() {
        None => {},
        Some(main_module) => {
            let _ = main_module.exports();
            let _ = main_module.require();
            let _ = main_module.id();
            let _ = main_module.filename();
            let _ = main_module.loaded();
            let _ = main_module.parent();
            let _ = main_module.children();
            let _ = main_module.paths();
        },
    }
}

#[wasm_bindgen_test]
fn memory_usage() {
    let memory_usage = process.memory_usage();
    let _ = memory_usage.external();
    let _ = memory_usage.heap_used();
    let _ = memory_usage.heap_total();
    let _ = memory_usage.rss();
}

#[wasm_bindgen_test]
fn next_tick() {
    use js_sys::Number;
    use wasm_bindgen::{prelude::*, JsCast};
    let clo = Closure::wrap(Box::new(|lhs: Number, rhs: Number| {
        let res = lhs.value_of() + rhs.value_of();
        res.into()
    }) as Box<dyn Fn(Number, Number) -> Number>);
    let add = clo.as_ref().unchecked_ref();
    let lhs = 1.into();
    let rhs = 1.into();
    process.next_tick(&add, Box::new([lhs, rhs]));
    clo.forget();
}

#[wasm_bindgen_test]
fn platform() {
    process.platform();
}

#[wasm_bindgen_test]
fn release() {
    let process_release = process.release();
    let _ = process_release.name();
    let _ = process_release.source_url();
    let _ = process_release.headers_url();
    let _ = process_release.lib_url();
    let _ = process_release.lts();
}

// FIXME: see binding comment; might be undefined
#[wasm_bindgen_test]
fn send() {
    // use wasm_bindgen::prelude::*;
    // let message = JsValue::UNDEFINED;
    // let send_handle = Default::default();
    // let options = Default::default();
    // let callback = Default::default();
    // process.send(&message, send_handle, options, callback);
}

#[wasm_bindgen_test]
fn set_egid() {
    if let Err(_err) = process.set_egid(0) {}
}

#[wasm_bindgen_test]
fn set_euid() {
    if let Err(_err) = process.set_euid(0) {}
}

#[wasm_bindgen_test]
fn set_gid() {
    if let Err(_err) = process.set_gid(0) {}
}

#[wasm_bindgen_test]
fn set_groups() {
    if let Err(_err) = process.set_groups(0) {}
}

#[wasm_bindgen_test]
fn set_uid() {
    if let Err(_err) = process.set_uid(0) {}
}

#[wasm_bindgen_test]
fn stderr() {
    process.stderr();
}

#[wasm_bindgen_test]
fn stdin() {
    process.stdin();
}

#[wasm_bindgen_test]
fn stdout() {
    process.stdout();
}

#[wasm_bindgen_test]
fn uptime() {
    process.uptime();
}

#[wasm_bindgen_test]
fn version() {
    process.version();
}

#[wasm_bindgen_test]
fn versions() {
    let versions = process.versions();
    let _ = versions.ares();
    let _ = versions.brotli();
    let _ = versions.cldr();
    let _ = versions.icu();
    let _ = versions.llhttp();
    let _ = versions.modules();
    let _ = versions.napi();
    let _ = versions.nghttp2();
    let _ = versions.node();
    let _ = versions.openssl();
    let _ = versions.tz();
    let _ = versions.unicode();
    let _ = versions.uv();
    let _ = versions.v8();
    let _ = versions.zlib();
}
