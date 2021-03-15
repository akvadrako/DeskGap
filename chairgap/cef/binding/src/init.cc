#include "init.h"
#include "application.h"

#include <include/cef_app.h>
#include <include/wrapper/cef_library_loader.h>

namespace {
    
}

extern "C" {
    int dgcef_init(const char* lib_path, int argc_arg, char** argv_arg) {
#ifdef __APPLE__
        dgcef_mac_init_application();
#endif

#ifdef _WIN32
        CefMainArgs args(::GetModuleHandle(nullptr));
#else
        CefMainArgs args(argc_arg, argv_arg);
#endif


    }
}
