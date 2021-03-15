#include "browser.h"
#include <include/cef_browser.h>

extern "C" {

struct dgcef_browser_s {
    CefRefPtr<CefBrowser> ref_ptr;
};

dgcef_browser_t dgcef_browser_new(void* parent_handle) {
    return new dgcef_browser_s { nullptr };
}

void dgcef_browser_free(dgcef_browser_t browser) {
    delete browser;
}

}
