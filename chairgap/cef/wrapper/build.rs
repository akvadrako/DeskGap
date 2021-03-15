use std::path::PathBuf;

pub fn ohyeah() {}

static LIBCEF_SRCS: &[&str] = &["shutdown_checker.cc", "transfer_util.cc"];
static LIBCEF_BASE_SRCS: &[&str] = &[
    "base/cef_atomicops_x86_gcc.cc",
    "base/cef_bind_helpers.cc",
    "base/cef_callback_internal.cc",
    "base/cef_lock.cc",
    "base/cef_lock_impl.cc",
    "base/cef_logging.cc",
    "base/cef_ref_counted.cc",
    "base/cef_string16.cc",
    "base/cef_thread_checker_impl.cc",
    "base/cef_weak_ptr.cc",
];

static LIBCEF_CPPTOC_SRCS: &[&str] = &[
    "cpptoc/accessibility_handler_cpptoc.cc",
    "cpptoc/app_cpptoc.cc",
    "cpptoc/audio_handler_cpptoc.cc",
    "cpptoc/base_ref_counted_cpptoc.cc",
    "cpptoc/base_scoped_cpptoc.cc",
    "cpptoc/browser_process_handler_cpptoc.cc",
    "cpptoc/client_cpptoc.cc",
    "cpptoc/completion_callback_cpptoc.cc",
    "cpptoc/context_menu_handler_cpptoc.cc",
    "cpptoc/cookie_access_filter_cpptoc.cc",
    "cpptoc/cookie_visitor_cpptoc.cc",
    "cpptoc/delete_cookies_callback_cpptoc.cc",
    "cpptoc/dev_tools_message_observer_cpptoc.cc",
    "cpptoc/dialog_handler_cpptoc.cc",
    "cpptoc/display_handler_cpptoc.cc",
    "cpptoc/domvisitor_cpptoc.cc",
    "cpptoc/download_handler_cpptoc.cc",
    "cpptoc/download_image_callback_cpptoc.cc",
    "cpptoc/drag_handler_cpptoc.cc",
    "cpptoc/end_tracing_callback_cpptoc.cc",
    "cpptoc/extension_handler_cpptoc.cc",
    "cpptoc/find_handler_cpptoc.cc",
    "cpptoc/focus_handler_cpptoc.cc",
    "cpptoc/jsdialog_handler_cpptoc.cc",
    "cpptoc/keyboard_handler_cpptoc.cc",
    "cpptoc/life_span_handler_cpptoc.cc",
    "cpptoc/load_handler_cpptoc.cc",
    "cpptoc/media_observer_cpptoc.cc",
    "cpptoc/media_route_create_callback_cpptoc.cc",
    "cpptoc/media_sink_device_info_callback_cpptoc.cc",
    "cpptoc/menu_model_delegate_cpptoc.cc",
    "cpptoc/navigation_entry_visitor_cpptoc.cc",
    "cpptoc/pdf_print_callback_cpptoc.cc",
    "cpptoc/print_handler_cpptoc.cc",
    "cpptoc/read_handler_cpptoc.cc",
    "cpptoc/register_cdm_callback_cpptoc.cc",
    "cpptoc/render_handler_cpptoc.cc",
    "cpptoc/render_process_handler_cpptoc.cc",
    "cpptoc/request_context_handler_cpptoc.cc",
    "cpptoc/request_handler_cpptoc.cc",
    "cpptoc/resolve_callback_cpptoc.cc",
    "cpptoc/resource_bundle_handler_cpptoc.cc",
    "cpptoc/resource_handler_cpptoc.cc",
    "cpptoc/resource_request_handler_cpptoc.cc",
    "cpptoc/response_filter_cpptoc.cc",
    "cpptoc/run_file_dialog_callback_cpptoc.cc",
    "cpptoc/scheme_handler_factory_cpptoc.cc",
    "cpptoc/server_handler_cpptoc.cc",
    "cpptoc/set_cookie_callback_cpptoc.cc",
    "cpptoc/string_visitor_cpptoc.cc",
    "cpptoc/task_cpptoc.cc",
    "cpptoc/urlrequest_client_cpptoc.cc",
    "cpptoc/v8accessor_cpptoc.cc",
    "cpptoc/v8array_buffer_release_callback_cpptoc.cc",
    "cpptoc/v8handler_cpptoc.cc",
    "cpptoc/v8interceptor_cpptoc.cc",
    "cpptoc/web_plugin_info_visitor_cpptoc.cc",
    "cpptoc/web_plugin_unstable_callback_cpptoc.cc",
    "cpptoc/write_handler_cpptoc.cc",
];

static LIBCEF_CPPTOC_VIEWS_SRCS: &[&str] = &[
    "cpptoc/views/browser_view_delegate_cpptoc.cc",
    "cpptoc/views/button_delegate_cpptoc.cc",
    "cpptoc/views/menu_button_delegate_cpptoc.cc",
    "cpptoc/views/panel_delegate_cpptoc.cc",
    "cpptoc/views/textfield_delegate_cpptoc.cc",
    "cpptoc/views/view_delegate_cpptoc.cc",
    "cpptoc/views/window_delegate_cpptoc.cc",
];

static LIBCEF_CTOCPP_SRCS: &[&str] = &[
    "ctocpp/auth_callback_ctocpp.cc",
    "ctocpp/before_download_callback_ctocpp.cc",
    "ctocpp/binary_value_ctocpp.cc",
    "ctocpp/browser_ctocpp.cc",
    "ctocpp/browser_host_ctocpp.cc",
    "ctocpp/callback_ctocpp.cc",
    "ctocpp/command_line_ctocpp.cc",
    "ctocpp/context_menu_params_ctocpp.cc",
    "ctocpp/cookie_manager_ctocpp.cc",
    "ctocpp/dictionary_value_ctocpp.cc",
    "ctocpp/domdocument_ctocpp.cc",
    "ctocpp/domnode_ctocpp.cc",
    "ctocpp/download_item_callback_ctocpp.cc",
    "ctocpp/download_item_ctocpp.cc",
    "ctocpp/drag_data_ctocpp.cc",
    "ctocpp/extension_ctocpp.cc",
    "ctocpp/file_dialog_callback_ctocpp.cc",
    "ctocpp/frame_ctocpp.cc",
    "ctocpp/get_extension_resource_callback_ctocpp.cc",
    "ctocpp/image_ctocpp.cc",
    "ctocpp/jsdialog_callback_ctocpp.cc",
    "ctocpp/list_value_ctocpp.cc",
    "ctocpp/media_route_ctocpp.cc",
    "ctocpp/media_router_ctocpp.cc",
    "ctocpp/media_sink_ctocpp.cc",
    "ctocpp/media_source_ctocpp.cc",
    "ctocpp/menu_model_ctocpp.cc",
    "ctocpp/navigation_entry_ctocpp.cc",
    "ctocpp/post_data_ctocpp.cc",
    "ctocpp/post_data_element_ctocpp.cc",
    "ctocpp/print_dialog_callback_ctocpp.cc",
    "ctocpp/print_job_callback_ctocpp.cc",
    "ctocpp/print_settings_ctocpp.cc",
    "ctocpp/process_message_ctocpp.cc",
    "ctocpp/registration_ctocpp.cc",
    "ctocpp/request_callback_ctocpp.cc",
    "ctocpp/request_context_ctocpp.cc",
    "ctocpp/request_ctocpp.cc",
    "ctocpp/resource_bundle_ctocpp.cc",
    "ctocpp/resource_read_callback_ctocpp.cc",
    "ctocpp/resource_skip_callback_ctocpp.cc",
    "ctocpp/response_ctocpp.cc",
    "ctocpp/run_context_menu_callback_ctocpp.cc",
    "ctocpp/scheme_registrar_ctocpp.cc",
    "ctocpp/select_client_certificate_callback_ctocpp.cc",
    "ctocpp/server_ctocpp.cc",
    "ctocpp/sslinfo_ctocpp.cc",
    "ctocpp/sslstatus_ctocpp.cc",
    "ctocpp/stream_reader_ctocpp.cc",
    "ctocpp/stream_writer_ctocpp.cc",
    "ctocpp/task_runner_ctocpp.cc",
    "ctocpp/thread_ctocpp.cc",
    "ctocpp/urlrequest_ctocpp.cc",
    "ctocpp/v8context_ctocpp.cc",
    "ctocpp/v8exception_ctocpp.cc",
    "ctocpp/v8stack_frame_ctocpp.cc",
    "ctocpp/v8stack_trace_ctocpp.cc",
    "ctocpp/v8value_ctocpp.cc",
    "ctocpp/value_ctocpp.cc",
    "ctocpp/waitable_event_ctocpp.cc",
    "ctocpp/web_plugin_info_ctocpp.cc",
    "ctocpp/x509cert_principal_ctocpp.cc",
    "ctocpp/x509certificate_ctocpp.cc",
    "ctocpp/xml_reader_ctocpp.cc",
    "ctocpp/zip_reader_ctocpp.cc",
];

static LIBCEF_CTOCPP_VIEWS_SRCS: &[&str] = &[
    "ctocpp/views/box_layout_ctocpp.cc",
    "ctocpp/views/browser_view_ctocpp.cc",
    "ctocpp/views/button_ctocpp.cc",
    "ctocpp/views/display_ctocpp.cc",
    "ctocpp/views/fill_layout_ctocpp.cc",
    "ctocpp/views/label_button_ctocpp.cc",
    "ctocpp/views/layout_ctocpp.cc",
    "ctocpp/views/menu_button_ctocpp.cc",
    "ctocpp/views/menu_button_pressed_lock_ctocpp.cc",
    "ctocpp/views/panel_ctocpp.cc",
    "ctocpp/views/scroll_view_ctocpp.cc",
    "ctocpp/views/textfield_ctocpp.cc",
    "ctocpp/views/view_ctocpp.cc",
    "ctocpp/views/window_ctocpp.cc",
];

static LIBCEF_WRAPPER_SRCS: &[&str] = &[
    "wrapper/cef_byte_read_handler.cc",
    "wrapper/cef_closure_task.cc",
    "wrapper/cef_message_router.cc",
    "wrapper/cef_resource_manager.cc",
    "wrapper/cef_scoped_temp_dir.cc",
    "wrapper/cef_stream_resource_handler.cc",
    "wrapper/cef_xml_object.cc",
    "wrapper/cef_zip_archive.cc",
    "wrapper/libcef_dll_wrapper.cc",
    "wrapper/libcef_dll_wrapper2.cc",
    "wrapper/cef_library_loader.cc",
    "wrapper/libcef_dll_dylib.cc",
];

fn main() {
    // let target_env = std::env::var("CARGO_CFG_TARGET_ENV").unwrap();
    let mut cc_build = cc::Build::new();

    let src_prefix = PathBuf::from("src/libcef_dll");

    cc_build
        .cpp(true)
        .warnings(false)
        .flag("-std=gnu++11")
        .flag("-fno-exceptions")
        .flag("-fno-rtti")
        .flag("-Wno-deprecated-declarations")
        .flag("-Wno-undefined-var-template")
        .define("WRAPPING_CEF_SHARED", None)
        .static_crt(true)
        .files(
            [
                LIBCEF_SRCS,
                LIBCEF_BASE_SRCS,
                LIBCEF_CPPTOC_SRCS,
                LIBCEF_CPPTOC_VIEWS_SRCS,
                LIBCEF_CTOCPP_SRCS,
                LIBCEF_CTOCPP_VIEWS_SRCS,
                LIBCEF_WRAPPER_SRCS,
            ]
            .concat()
            .iter()
            .map(|p| src_prefix.join(p)),
        )
        .include("src");
    cc_build.compile("cef_dll_wrapper");
}
