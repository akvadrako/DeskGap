#import <AppKit/AppKit.h>

#include <include/cef_application_mac.h>

@interface CefApplication : NSApplication <CefAppProtocol> {
    @private BOOL handlingSendEvent_;
}
@end

@implementation CefApplication
- (BOOL)isHandlingSendEvent {
    return handlingSendEvent_;
}

- (void)setHandlingSendEvent:(BOOL)handlingSendEvent {
    handlingSendEvent_ = handlingSendEvent;
}

- (void)sendEvent:(NSEvent*)event {
    CefScopedSendingEvent sendingEventScoper;
    [super sendEvent:event];
}

@end

extern "C" void rust_cef_mac_init_application() {
    [CefApplication sharedApplication];
}

