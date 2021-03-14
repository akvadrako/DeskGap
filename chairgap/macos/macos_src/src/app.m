#include "app.h"

#import <AppKit/AppKit.h>

@interface DGAppDelegate: NSObject <NSApplicationDelegate>
-(instancetype)initWithEventHandler: (DGAppEventHandler) event_handler;
@end

@implementation DGAppDelegate {
    DGAppEventHandler event_handler_;
}

-(instancetype)initWithEventHandler: (DGAppEventHandler) event_handler {
    self = [super init];
    if (self) {
        event_handler_ = event_handler;
    }
    return self;
}

- (void)applicationWillFinishLaunching:(NSNotification *)aNotification {
    event_handler_(DGAppEventTypeWillLaunch, NULL);
}

- (BOOL)applicationShouldTerminateAfterLastWindowClosed:(NSApplication *)sender {
    return NO;
}

- (BOOL)applicationShouldHandleReopen:(NSApplication*)theApplication hasVisibleWindows:(BOOL)flag {
    event_handler_(DGAppEventTypeReopen, NULL);
    return NO;
}

- (NSApplicationTerminateReply)applicationShouldTerminate:(NSApplication *)sender {
    NSApplicationTerminateReply reply;
    event_handler_(DGAppEventTypeShouldClose, &reply);
    return reply;
}
@end

id DGAppDelegateNew(DGAppEventHandler event_handler) {
    return [[DGAppDelegate alloc] initWithEventHandler: event_handler];
}

void DGAppTick() {
    NSApplication* app = [NSApplication sharedApplication];
    @autoreleasepool {
        NSEvent *event = [app
            nextEventMatchingMask:NSEventMaskAny
            untilDate:[NSDate distantFuture]
            inMode:NSDefaultRunLoopMode
            dequeue:YES
        ];
        [app sendEvent: event];
        [app updateWindows];
    }
}
