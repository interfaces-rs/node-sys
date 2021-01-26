use node_sys::os;
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
pub fn arch() {
    os::arch();
}

#[wasm_bindgen_test]
pub fn constants() {
    let _ = os::constants;
}

#[wasm_bindgen_test]
pub fn constants_dlopen() {
    let dlopen = os::constants.dlopen();
    // let _ = dlopen.RTLD_DEEPBIND();
    let _ = dlopen.RTLD_GLOBAL();
    let _ = dlopen.RTLD_LAZY();
    let _ = dlopen.RTLD_LOCAL();
    let _ = dlopen.RTLD_NOW();
}

#[wasm_bindgen_test]
pub fn constants_errno() {
    let errno = os::constants.errno();
    let _ = errno.E2BIG();
    let _ = errno.EACCES();
    let _ = errno.EADDRINUSE();
    let _ = errno.EADDRNOTAVAIL();
    let _ = errno.EAFNOSUPPORT();
    let _ = errno.EAGAIN();
    let _ = errno.EALREADY();
    let _ = errno.EBADF();
    let _ = errno.EBADMSG();
    let _ = errno.EBUSY();
    let _ = errno.ECANCELED();
    let _ = errno.ECHILD();
    let _ = errno.ECONNABORTED();
    let _ = errno.ECONNREFUSED();
    let _ = errno.ECONNRESET();
    let _ = errno.EDEADLK();
    let _ = errno.EDESTADDRREQ();
    let _ = errno.EDOM();
    let _ = errno.EDQUOT();
    let _ = errno.EEXIST();
    let _ = errno.EFAULT();
    let _ = errno.EFBIG();
    let _ = errno.EHOSTUNREACH();
    let _ = errno.EIDRM();
    let _ = errno.EILSEQ();
    let _ = errno.EINPROGRESS();
    let _ = errno.EINTR();
    let _ = errno.EINVAL();
    let _ = errno.EIO();
    let _ = errno.EISCONN();
    let _ = errno.EISDIR();
    let _ = errno.ELOOP();
    let _ = errno.EMFILE();
    let _ = errno.EMLINK();
    let _ = errno.EMSGSIZE();
    let _ = errno.EMULTIHOP();
    let _ = errno.ENAMETOOLONG();
    let _ = errno.ENETDOWN();
    let _ = errno.ENETRESET();
    let _ = errno.ENETUNREACH();
    let _ = errno.ENFILE();
    let _ = errno.ENOBUFS();
    let _ = errno.ENODATA();
    let _ = errno.ENODEV();
    let _ = errno.ENOENT();
    let _ = errno.ENOEXEC();
    let _ = errno.ENOLCK();
    let _ = errno.ENOLINK();
    let _ = errno.ENOMEM();
    let _ = errno.ENOMSG();
    let _ = errno.ENOPROTOOPT();
    let _ = errno.ENOSPC();
    let _ = errno.ENOSR();
    let _ = errno.ENOSTR();
    let _ = errno.ENOSYS();
    let _ = errno.ENOTCONN();
    let _ = errno.ENOTDIR();
    let _ = errno.ENOTEMPTY();
    let _ = errno.ENOTSOCK();
    let _ = errno.ENOTSUP();
    let _ = errno.ENOTTY();
    let _ = errno.ENXIO();
    let _ = errno.EOPNOTSUPP();
    let _ = errno.EOVERFLOW();
    let _ = errno.EPERM();
    let _ = errno.EPIPE();
    let _ = errno.EPROTO();
    let _ = errno.EPROTONOSUPPORT();
    let _ = errno.EPROTOTYPE();
    let _ = errno.ERANGE();
    let _ = errno.EROFS();
    let _ = errno.ESPIPE();
    let _ = errno.ESRCH();
    let _ = errno.ESTALE();
    let _ = errno.ETIME();
    let _ = errno.ETIMEDOUT();
    let _ = errno.ETXTBSY();
    let _ = errno.EWOULDBLOCK();
    let _ = errno.EXDEV();
    cfg_if::cfg_if! {
        if #[cfg(windows)] {
            let _ = errno.WSAEINTR();
            let _ = errno.WSAEBADF();
            let _ = errno.WSAEACCESS();
            let _ = errno.WASEFAULT();
            let _ = errno.WSAEINVAL();
            let _ = errno.WASEMFILE();
            let _ = errno.WSAEWOULDBLOCK();
            let _ = errno.WSAEINPROGRESS();
            let _ = errno.WSAEALREADY();
            let _ = errno.WSAENOTSOCK();
            let _ = errno.WSAEDESTADDRREQ();
            let _ = errno.WSAEMSGSIZE();
            let _ = errno.WSAEPROTOTYPE();
            let _ = errno.WSAENOPROTOOPT();
            let _ = errno.WSAEPROTONOSUPPORT();
            let _ = errno.WSAESOCKTNOSUPPORT();
        }
    }
}

// FIXME
// #[wasm_bindgen_test]
// pub fn constants_libuv() {
//     let libuv = os::constants.libuv();
//     libuv.UV_UDP_REUSEADDR();
// }

#[wasm_bindgen_test]
pub fn constants_priority() {
    let priority = os::constants.priority();
    let _ = priority.PRIORITY_LOW();
    let _ = priority.PRIORITY_BELOW_NORMAL();
    let _ = priority.PRIORITY_NORMAL();
    let _ = priority.PRIORITY_ABOVE_NORMAL();
    let _ = priority.PRIORITY_HIGH();
    let _ = priority.PRIORITY_HIGHEST();
}

#[wasm_bindgen_test]
pub fn constants_signal() {
    let _ = os::constants.signal();
}

#[wasm_bindgen_test]
pub fn cpus() {
    let cpus = os::cpus();
    let cpu = cpus[0].clone().unchecked_into::<node_sys::CpuInfo>();
    let _ = cpu.model();
    let _ = cpu.speed();
    let times = cpu.times();
    let _ = times.idle();
    let _ = times.irq();
    let _ = times.nice();
    let _ = times.sys();
    let _ = times.user();
}

#[wasm_bindgen_test]
pub fn endianness() {
    os::endianness();
}

#[wasm_bindgen_test]
pub fn freemem() {
    os::freemem();
}

#[wasm_bindgen_test]
pub fn get_priority() {
    let pid = Default::default();
    os::get_priority(pid);
}

#[wasm_bindgen_test]
pub fn homedir() {
    os::homedir();
}

#[wasm_bindgen_test]
pub fn hostname() {
    os::hostname();
}

#[wasm_bindgen_test]
pub fn loadavg() {
    os::loadavg();
}

#[wasm_bindgen_test]
pub fn network_interfaces() {
    os::network_interfaces();
}

#[wasm_bindgen_test]
pub fn platform() {
    os::platform();
}

#[wasm_bindgen_test]
pub fn release() {
    os::release();
}

#[wasm_bindgen_test]
pub fn set_priority() {
    let priority = 0;
    os::set_priority(priority);
}

#[wasm_bindgen_test]
pub fn set_priority_for_pid() {
    let pid = Default::default();
    let priority = 0;
    os::set_priority_for_pid(pid, priority);
}

#[wasm_bindgen_test]
pub fn tmpdir() {
    os::tmpdir();
}

#[wasm_bindgen_test]
pub fn totalmem() {
    os::totalmem();
}

#[wasm_bindgen_test]
pub fn type_() {
    os::type_();
}

#[wasm_bindgen_test]
pub fn uptime() {
    os::uptime();
}

#[wasm_bindgen_test]
pub fn user_info() {
    let options = None;
    os::user_info(options);
}

#[allow(non_snake_case)]
#[wasm_bindgen_test]
pub fn EOL() {
    let _ = os::EOL.clone();
}
