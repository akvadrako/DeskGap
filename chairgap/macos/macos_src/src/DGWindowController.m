#import <AppKit/AppKit.h>

#import "window.h"

// @interface DGWindowContentView: NSView @end
// @implementation DGWindowContentView
// - (BOOL)isFlipped { return YES; }
// @end


@interface DGWindow: NSWindow
-(void)dg_startDragging;
@end
@implementation DGWindow {
    NSEvent* lastLeftMouseDownEvent_;
}
-(void)dg_startDragging {
    if (!lastLeftMouseDownEvent_) return;
    [self performWindowDragWithEvent: lastLeftMouseDownEvent_];
}
- (void)sendEvent: (NSEvent*)event {
    switch (event.type) {
    case NSEventTypeLeftMouseDown:
        lastLeftMouseDownEvent_ = event;
        break;
    case NSEventTypeLeftMouseUp:
        lastLeftMouseDownEvent_ = nil;
        break;
    default:
        break;
    }
    [super sendEvent: event];
}
@end

@interface DGWindowController: NSObject <NSWindowDelegate>
@end

@implementation DGWindowController {
    DGWindow* window_;
    NSView* webViewContainer_;
	DGWindowEventHandler eventHandler_;
	NSMutableArray<NSVisualEffectView*>* visualEffectViews_;
    NSMutableArray<NSLayoutConstraint*>* visualEffectViewSizeConstraints_;

    BOOL titleBarVisible_;
    BOOL customizesTrafficLightPosition_;
    NSPoint trafficLightPosition_;
    BOOL exitingFullScreen_;

    BOOL trafficLightVisible_;
}


-(NSRect)rectToVisiblePortion:(NSRect)rect {
	NSScreen* screen = [window_ screen];
    if (!screen) {
    	screen = [NSScreen mainScreen];
    }
	NSRect visibleFrame = [screen visibleFrame];
    rect.origin.x -= visibleFrame.origin.x;
    rect.origin.y -= visibleFrame.origin.y;
    rect.origin.y = visibleFrame.size.height - rect.origin.y - rect.size.height;
    return rect;
}
-(NSRect)rectFromVisiblePortion:(NSRect)rect {
	NSScreen* screen = [window_ screen];
    if (!screen) {
    	screen = [NSScreen mainScreen];
    }
	NSRect visibleFrame = [screen visibleFrame];
    rect.origin.y = visibleFrame.size.height - rect.origin.y - rect.size.height;
    rect.origin.x += visibleFrame.origin.x;
    rect.origin.y += visibleFrame.origin.y;
    return rect;
}

//Credits: https://github.com/electron/electron/blob/5ae3d1a1b2dbe11d3091d366467591d9cb21fdfe/atom/browser/native_window_mac.mm#L1479
-(void)setStyleMask:(NSWindowStyleMask)flag on:(bool)on {
    BOOL wasMaximizable = [[window_ standardWindowButton:NSWindowZoomButton] isEnabled];

    if (on) {
        window_.styleMask |= flag;
    }
    else {
        window_.styleMask &= ~flag;
    }

    [[window_ standardWindowButton: NSWindowZoomButton] setEnabled: wasMaximizable];
}
-(instancetype)initWithEventHandler: (DGWindowEventHandler) eventHandler {
    self = [super init];
    if (self) {
        eventHandler_ = eventHandler;
        window_ = [[DGWindow alloc]
            initWithContentRect: NSZeroRect
            styleMask: NSWindowStyleMaskTitled |
                NSWindowStyleMaskClosable |
                NSWindowStyleMaskMiniaturizable |
                NSWindowStyleMaskResizable
            backing: NSBackingStoreBuffered defer: NO
        ];
        [window_ setReleasedWhenClosed: NO];
        [window_ setBackgroundColor: [NSColor whiteColor]];
		[window_ setTabbingMode: NSWindowTabbingModeDisallowed];

        NSView* visualEffectContainer = [[NSView alloc] initWithFrame: window_.contentView.bounds];
        visualEffectContainer.autoresizingMask = NSViewWidthSizable | NSViewHeightSizable;
        [window_.contentView addSubview: visualEffectContainer];

        NSLayoutAttribute edgeAttributes[4] = { NSLayoutAttributeBottom, NSLayoutAttributeRight, NSLayoutAttributeTop, NSLayoutAttributeLeft };

        NSMutableArray<NSLayoutConstraint*>* allConstraints = [NSMutableArray arrayWithCapacity: 4 * 4];
        visualEffectViews_ = [NSMutableArray arrayWithCapacity: 4];
        visualEffectViewSizeConstraints_ = [NSMutableArray arrayWithCapacity: 4];
        for (int viewIdx = 0; viewIdx < 4; ++viewIdx) {
            NSVisualEffectView* view = [[NSVisualEffectView alloc] initWithFrame: visualEffectContainer.bounds];
            view.translatesAutoresizingMaskIntoConstraints = NO;
            [view setHidden: YES];
            [view setMaterial: NSVisualEffectMaterialSidebar];
            [visualEffectViews_ addObject: view];
            [visualEffectContainer addSubview: view];
            for (int edgeIdx = 0; edgeIdx < 4; ++edgeIdx) {
                NSLayoutConstraint* constraint = nil;
                if (viewIdx != edgeIdx) {
                    constraint = [NSLayoutConstraint
                        constraintWithItem: view attribute: edgeAttributes[edgeIdx]
                        relatedBy: NSLayoutRelationEqual
                        toItem: visualEffectContainer attribute: edgeAttributes[edgeIdx]
                        multiplier: 1 constant: 0
                    ];
                }
                else {
                    NSLayoutAttribute sizeAttribute = (viewIdx % 2 == 0) ? NSLayoutAttributeHeight : NSLayoutAttributeWidth;
                    constraint = [NSLayoutConstraint
                        constraintWithItem: view attribute: sizeAttribute
                        relatedBy: NSLayoutRelationEqual
                        toItem: nil attribute: NSLayoutAttributeNotAnAttribute
                        multiplier: 0 constant: 0
                    ];
                    [visualEffectViewSizeConstraints_ addObject: constraint];
                }
                [allConstraints addObject: constraint];
            }
        }
        [NSLayoutConstraint activateConstraints: allConstraints];

        webViewContainer_ = [[NSView alloc] initWithFrame: window_.contentView.bounds];
        webViewContainer_.autoresizingMask = NSViewWidthSizable | NSViewHeightSizable;
        [window_.contentView addSubview: webViewContainer_];  

        [self setTitleBarVisible: true];
        [self setTrafficLightVisible: true];
        [self setCustomized: NO trafficLightPosition: NSZeroPoint];  

        [[window_ standardWindowButton: NSWindowZoomButton] setEnabled: YES];
     

		[window_ setDelegate: self];
    }
    return self;
}


- (void)windowDidBecomeKey:(NSNotification *)notification {
    eventHandler_(DGWindowEventTypeFocus);
}
- (void)windowDidResignKey:(NSNotification *)notification {
    eventHandler_(DGWindowEventTypeBlur);
}
- (void)windowDidMove:(NSNotification *)notification {
    eventHandler_(DGWindowEventTypeMove);
}
- (void)windowDidResize:(NSNotification *)notification {
    [self updateTrafficLightPosition];
    eventHandler_(DGWindowEventTypeResize);
}
- (BOOL)windowShouldClose:(NSWindow *)sender {
    eventHandler_(DGWindowEventTypeClose);
    return NO;
}

- (void)windowWillExitFullScreen:(NSNotification *)notification {
    exitingFullScreen_ = YES;
    [self updateTrafficLightPosition];
}
- (void)windowDidExitFullScreen:(NSNotification *)notification {
    exitingFullScreen_ = NO;
    [self updateTrafficLightPosition];
}

-(void)setVisible: (BOOL)visible {
    if (visible) {
        [[NSApplication sharedApplication] activateIgnoringOtherApps: YES];
        [window_ makeKeyAndOrderFront: nil];
    }
    else {
        [window_ orderOut: nil];
    }
}
-(BOOL)visible {
    return [window_ isVisible];
}

-(NSRect)frame {
    return [self rectToVisiblePortion: [window_ frame]];
}

// TODO: animcation
-(void)setLocation: (const NSPoint*)location center: (BOOL)center size: (const NSSize*)size withAnimationFinished: (void(^)())callback {
    NSRect frame = [self rectToVisiblePortion: [window_ frame]];

    if (location != NULL) {
        frame.origin = *location;
    }
    if (size != NULL) {
        frame.size = *size;
    }

    [window_ setFrame: [self rectFromVisiblePortion: frame] display: YES animate: NO];

    if (center) {
        [window_ center];
    }

    if (callback != nil) {
        callback();
    }
}

-(void)setClosable: (BOOL)closable {
    [self setStyleMask: NSWindowStyleMaskClosable on: closable];
}
-(BOOL)closable {
    return ([window_ styleMask] & NSWindowStyleMaskClosable) != 0;
}

-(void)setResizable: (BOOL)resizable {
    [self setStyleMask: NSWindowStyleMaskResizable on: resizable];
}
-(BOOL)resizable {
    return ([window_ styleMask] & NSWindowStyleMaskResizable) != 0;
}

-(void)setMinimizable: (BOOL)minimizable {
    [self setStyleMask: NSWindowStyleMaskMiniaturizable on: minimizable];
}
-(BOOL)minimizable {
    return ([window_ styleMask] & NSWindowStyleMaskMiniaturizable) != 0;
}

-(void)setTitle: (NSString*)title {
    [window_ setTitle: title];
    [self updateTrafficLightPosition];
}
-(NSString*)title {
    return [window_ title];
}

-(NSSize)size {
    return [window_ frame].size;
}

-(NSPoint)position {
    NSRect frame = [self rectToVisiblePortion: [window_ frame]];
    return frame.origin;
}

-(void)setMaxSize: (NSSize)size {
    [window_ setMaxSize: size];
}

-(NSSize)maxSize {
    return [window_ maxSize];
}

-(void)setMinSize: (NSSize)size {
    [window_ setMinSize: size];
}

-(NSSize)minSize {
    return [window_ minSize];
}

-(NSView*)webViewContainer {
    return webViewContainer_;
}

-(void)startDragging {
    [window_ dg_startDragging];
}

-(void)close {
    [window_ close];
}

-(void)setVisualEffectViewTop:
    (DGVisualEffectViewConfig)topConfig
    left: (DGVisualEffectViewConfig)leftConfig
    bottom: (DGVisualEffectViewConfig)bottomConfig
    right: (DGVisualEffectViewConfig)rightConfig {
    DGVisualEffectViewConfig configs[4] = { topConfig, leftConfig, bottomConfig, rightConfig};
    for (int i = 0; i < 4; ++i) {
        DGVisualEffectViewConfig config = configs[i];
        if (config.size == 0) {
            [visualEffectViews_[i] setHidden: YES];
        }
        else if (config.size > 0) {
            [visualEffectViews_[i] setHidden: NO];
            [visualEffectViewSizeConstraints_[i] setConstant: (CGFloat)config.size];
        }
    }
}

-(void)setTitleBarVisible: (BOOL)visible {
    titleBarVisible_ = visible;

    NSRect windowFrame = [window_ frame];
    if (visible) {
        [window_ setTitleVisibility: NSWindowTitleVisible];
        [window_ setTitlebarAppearsTransparent: NO];
        [self setStyleMask: NSWindowStyleMaskFullSizeContentView on: NO];
    }
    else {
        [window_ setTitleVisibility: NSWindowTitleHidden];
        [window_ setTitlebarAppearsTransparent: YES];
        [self setStyleMask: NSWindowStyleMaskFullSizeContentView on: YES];
    }
    [window_ setFrame: windowFrame display: YES];
    [self updateTrafficLightVisibility];
    [self updateTrafficLightPosition];
}

-(void)setTrafficLightVisible: (BOOL)visible {
    trafficLightVisible_ = visible;
    [self updateTrafficLightVisibility];
}

-(void)setCustomized: (BOOL)customizes trafficLightPosition: (NSPoint)position {
    customizesTrafficLightPosition_ = customizes;
    trafficLightPosition_ = position;
    [self updateTrafficLightPosition];
}


-(void)updateTrafficLightPosition {
    if (titleBarVisible_ || !customizesTrafficLightPosition_) {
        return;
    }
    // [[window_ standardWindowButton: NSWindowZoomButton] setEnabled: maximizable];
    if ([window_ styleMask] & NSWindowStyleMaskFullScreen) {
        return;
    }

    NSButton* close = [window_ standardWindowButton:NSWindowCloseButton];
    NSButton* miniaturize = [window_ standardWindowButton:NSWindowMiniaturizeButton];
    NSButton* zoom = [window_ standardWindowButton:NSWindowZoomButton];
    NSView* titleBarContainerView = close.superview.superview;

    // Hide the container when exiting fullscreen, otherwise traffic light buttons
    // jump
    if (exitingFullScreen_) {
        [titleBarContainerView setHidden:YES];
        return;
    }

    [titleBarContainerView setHidden:NO];
    CGFloat buttonHeight = [close frame].size.height;
    CGFloat titleBarFrameHeight = buttonHeight + trafficLightPosition_.y;
    CGRect titleBarRect = titleBarContainerView.frame;
    titleBarRect.size.height = titleBarFrameHeight;
    titleBarRect.origin.y = window_.frame.size.height - titleBarFrameHeight;
    [titleBarContainerView setFrame:titleBarRect];

    NSArray* windowButtons = @[ close, miniaturize, zoom ];
    const CGFloat space_between =
            [miniaturize frame].origin.x - [close frame].origin.x;
    for (NSUInteger i = 0; i < windowButtons.count; i++) {
        NSView* view = [windowButtons objectAtIndex:i];
        CGRect rect = [view frame];
        rect.origin.x = trafficLightPosition_.x + (i * space_between);
        rect.origin.y = (titleBarFrameHeight - rect.size.height) / 2;
        [view setFrameOrigin:rect.origin];
    }
}

-(void)updateTrafficLightVisibility {
    const NSWindowButton buttonTypes[] = { NSWindowCloseButton, NSWindowMiniaturizeButton, NSWindowZoomButton };
    for (int i = 0; i < 3; i++) {
        [[window_ standardWindowButton: buttonTypes[i]] setHidden: !trafficLightVisible_];
    }
}

-(NSView*)contentView {
    return [window_ contentView];
}

@end

id DGWindowControllerNew(DGWindowEventHandler handler) {
    return [[DGWindowController alloc] initWithEventHandler: handler];
}

