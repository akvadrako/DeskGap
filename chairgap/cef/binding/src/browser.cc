#include "browser.h"
#include <include/cef_browser.h>
#include <include/cef_client.h>

namespace {
    class Client: public CefClient {
    public:
        Client() {}
    private:
        IMPLEMENT_REFCOUNTING(Client);
        DISALLOW_COPY_AND_ASSIGN(Client);
    };
}

extern "C" {

struct dgcef_browser_s {
    CefRefPtr<CefBrowser> browser_ptr;
};

dgcef_browser_t dgcef_browser_new(void* parent_handle) {
    CefWindowInfo window_info;
    CefBrowserSettings browser_settings;
    window_info.parent_view = parent_handle;
    CefRefPtr<CefClient> client = new Client;

    std::string url("http://www.baidu.com/");

    CefRefPtr<CefBrowser> browser_ptr = CefBrowserHost::CreateBrowserSync(
        window_info, client, url, browser_settings, nullptr, nullptr
    );
    return new dgcef_browser_s { browser_ptr };
}

void dgcef_browser_free(dgcef_browser_t browser) {
    delete browser;
}

}
