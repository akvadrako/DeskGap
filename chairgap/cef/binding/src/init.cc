#include "init.h"
#include "application.h"

#include <include/cef_app.h>
#include <include/wrapper/cef_library_loader.h>
#include <stdlib.h>
#ifdef _WIN32
#include <Windows.h>
#endif

namespace {
    class App: public CefApp {
    public:
        App() { }
    private:
        void OnBeforeCommandLineProcessing(
            const CefString& process_type,
            CefRefPtr<CefCommandLine> command_line
        ) override {
#ifdef __APPLE__
            if (process_type.empty()) {
                command_line->AppendSwitch("use-mock-keychain");
            }
#endif
        }
        IMPLEMENT_REFCOUNTING(App);
        DISALLOW_COPY_AND_ASSIGN(App);
    };

    #ifdef _WIN32
    std::wstring join_path(const wchar_t* first, const wchar_t* second) {
        std::wstring path(first);
        if ((path.empty() || path.back() != L'\\') && second[0] != '\\') {
            path += L"\\";
        }
        return path + second;
    }
    #else
    std::string join_path(const char* first, const char* second) {
        std::string path(first);
        if ((path.empty() || path.back() != '/') && second[0] != '/') {
            path += "/";
        }
        return path + second;
    }
    #endif

}

extern "C" {
    int dgcef_init(const void* cef_path, int argc_arg, char** argv_arg) {

#ifdef _WIN32
        std::string framework_path;
        std::wstring dylib_path = join_path((const wchar_t*)cef_path, L"libcef.dll");
        std::string helper_path;
#else
#ifdef __APPLE__
        std::string framework_path = join_path((const char*)cef_path, "Chromium Embedded Framework.framework");
        std::string dylib_path = join_path((const char*)cef_path, "Chromium Embedded Framework.framework/Chromium Embedded Framework");
        std::string helper_path = join_path((const char*)cef_path, "cefclient Helper.app/Contents/MacOS/cefclient Helper");
#else // LINUX
        std::string framework_path;
        std::string dylib_path = join_path((const char*)cef_path, "libcef.so");
        std::string helper_path;
#endif
#endif
        if (cef_load_library(dylib_path.c_str()) != 1) {
            return 0;
        }
#ifdef __APPLE__
        dgcef_mac_init_application();
#endif

#ifdef _WIN32
        CefMainArgs args(::GetModuleHandle(nullptr));
#else
        CefMainArgs args(argc_arg, argv_arg);
#endif

        if (helper_path.empty()) {
            CefRefPtr<CefApp> helper_app;
            int ret = CefExecuteProcess(args, helper_app, nullptr);
            if (ret != -1) {
                exit(ret);
            }
        }

        CefRefPtr<CefApp> app = new App;
        CefSettings settings;
        settings.no_sandbox = 1;
        if (!framework_path.empty()) {
            CefString(&settings.framework_dir_path).FromString(framework_path);
        }
        if (!helper_path.empty()) {
            CefString(&settings.browser_subprocess_path).FromString(helper_path);
        }
        bool init_ret = CefInitialize(args, settings, app, nullptr);
        if (!init_ret) {
            fprintf(stderr, "CefInitialize Failed.\n");
        }
        return init_ret ? 1 : 0;
    }

    void dgcef_run_message_loop() {
        CefRunMessageLoop();
    }
}
