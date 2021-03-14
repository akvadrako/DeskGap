mod strings;

use crate::strings::new_cef_string;
use chairgap_cef_sys::bindings::{
    cef_app_t, CefLibrary, _cef_app_t, _cef_base_ref_counted_t, _cef_browser_process_handler_t,
    _cef_command_line_t, _cef_main_args_t, _cef_render_process_handler_t,
    _cef_resource_bundle_handler_t, _cef_scheme_registrar_t, _cef_settings_t, cef_string_t,
};
use chairgap_common::engine::Engine;
use std::ffi::{c_void, CString, OsString};
use std::fmt::{Debug, Formatter};
use std::mem::size_of;
use std::os::raw::c_char;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::ptr::null_mut;

static CEF_FRAMEWORK_PATH: &str =
    "/Users/patr0nus/cef_g6f30454_client/Release/cefclient.app/Contents/Frameworks/Chromium Embedded Framework.framework";
static CEF_SUBPROCESS_PATH: &str =
    "/Users/patr0nus/cef_g6f30454_client/Release/cefclient.app/Contents/Frameworks/cefclient Helper.app/Contents/MacOS/cefclient Helper";

unsafe extern "C" fn on_before_command_line_processing(
    _app: *mut cef_app_t,
    _process_type: *const cef_string_t,
    _command_line: *mut _cef_command_line_t,
) {
}

unsafe extern "C" fn on_register_custom_schemes(
    _self: *mut cef_app_t,
    _registrar: *mut _cef_scheme_registrar_t,
) {
}

unsafe extern "C" fn get_resource_bundle_handler(
    _self: *mut cef_app_t,
) -> *mut _cef_resource_bundle_handler_t {
    // println!("get_resource_bundle_handler");
    null_mut()
}

unsafe extern "C" fn get_browser_process_handler(
    _self: *mut _cef_app_t,
) -> *mut _cef_browser_process_handler_t {
    // println!("get_browser_process_handler");
    null_mut()
}
unsafe extern "C" fn get_render_process_handler(
    _self: *mut _cef_app_t,
) -> *mut _cef_render_process_handler_t {
    println!("get_render_process_handler");
    null_mut()
}

fn leak_main_args(args: impl IntoIterator<Item = OsString>) -> _cef_main_args_t {
    let c_args = args.into_iter().map(|arg| {
        use std::os::unix::ffi::OsStrExt;
        CString::new(arg.as_bytes()).unwrap()
    });
    let c_arg_ptrs = c_args
        .map(|arg| arg.into_raw())
        .collect::<Vec<*mut c_char>>();

    _cef_main_args_t {
        argc: c_arg_ptrs.len() as i32,
        argv: c_arg_ptrs.leak().as_mut_ptr(),
    }
}

pub struct CefEngine {
    cef_lib: CefLibrary,
}
impl Debug for CefEngine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CefEngine").finish()
    }
}
impl Engine for CefEngine {
    unsafe fn run_event_loop(&self) -> bool {
        self.cef_lib.cef_run_message_loop();
        true
    }
    unsafe fn populate_web_view(&self, window_handle: *mut c_void) {
        println!("populate_web_view {:?}", window_handle)
    }
}

pub struct CefInitConfig<'a> {
    pub single_executable_mode: bool,
    pub path: &'a Path,
}

pub unsafe fn init_engine(config: CefInitConfig<'_>) -> anyhow::Result<CefEngine> {
    chairgap_cef_macos_src::dgcef_mac_init_application();
    let single_executable_mode = false;
    let framework_path = PathBuf::from(config.path)
        .join("Contents/Frameworks/Chromium Embedded Framework.framework");
    let helper_path = PathBuf::from(config.path)
        .join("Contents/Frameworks/cefclient Helper.app/Contents/MacOS/cefclient Helper");
    let cef_lib = CefLibrary::new(framework_path.join("Chromium Embedded Framework"))?;

    let app = Box::leak(Box::new(cef_app_t {
        base: _cef_base_ref_counted_t {
            size: size_of::<cef_app_t>() as u64,
            add_ref: None,
            release: None,
            has_one_ref: None,
            has_at_least_one_ref: None,
        },
        on_before_command_line_processing: Some(on_before_command_line_processing),
        on_register_custom_schemes: Some(on_register_custom_schemes),
        get_resource_bundle_handler: Some(get_resource_bundle_handler),
        get_browser_process_handler: Some(get_browser_process_handler),
        get_render_process_handler: Some(get_render_process_handler),
    }));

    if single_executable_mode {
        let main_args = Box::leak(Box::new(leak_main_args(std::env::args_os())));
        let ret = cef_lib.cef_execute_process(main_args, app, null_mut());
        if ret > 0 {
            std::process::exit(ret)
        }
    }
    let mut settings = _cef_settings_t::default();
    settings.size = size_of::<_cef_settings_t>() as u64;
    settings.framework_dir_path =
        new_cef_string(std::str::from_utf8(framework_path.as_os_str().as_bytes()).unwrap());
    if !single_executable_mode {
        settings.browser_subprocess_path =
            new_cef_string(std::str::from_utf8(helper_path.as_os_str().as_bytes()).unwrap());
    }
    settings.no_sandbox = 1;

    let settings = Box::leak(Box::new(settings));

    let main_args =
        Box::leak(Box::new(leak_main_args(std::env::args_os().take(1).chain(
            [OsString::from("--use-mock-keychain")].to_vec().drain(..),
        ))));
    let ret = cef_lib.cef_initialize(main_args, settings, app, null_mut());
    if ret == 0 {
        panic!("cef_initialize returned zero")
    }

    Ok(CefEngine { cef_lib })
}
