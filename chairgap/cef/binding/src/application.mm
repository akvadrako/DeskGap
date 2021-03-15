#import <AppKit/AppKit.h>

#include "application.h"
#include "include/cef_application_mac.h"

@interface DGCefApplication : NSApplication <CefAppProtocol> {
    @private BOOL handlingSendEvent_;
}
@end

@implementation DGCefApplication
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

void dgcef_mac_init_application() {
    [DGCefApplication sharedApplication];
}
