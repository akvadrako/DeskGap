#import <AppKit/AppKit.h>
#import <WebKit/WebKit.h>
#include "web_view.h"

@interface DGWebViewHandler: NSObject <WKURLSchemeHandler, WKScriptMessageHandler, WKUIDelegate> @end
@implementation DGWebViewHandler {
    DGWebViewEventHandler eventHandler_;
}
-(instancetype)initWithEventHandler: (DGWebViewEventHandler)eventHandler {
    self = [super init];
    if (self) eventHandler_ = eventHandler;
    return self;
}
- (void)webView:(WKWebView *)web_view startURLSchemeTask:(id<WKURLSchemeTask>)url_scheme_task {
    eventHandler_(DGWebViewEventTypeStartURLSchemeTask, url_scheme_task);
}
- (void)webView:(WKWebView *)web_view stopURLSchemeTask:(id<WKURLSchemeTask>)url_scheme_task {
    eventHandler_(DGWebViewEventTypeEndURLSchemeTask, url_scheme_task);
}
- (void)userContentController:(WKUserContentController*)userContentController didReceiveScriptMessage:(WKScriptMessage *)message {
    eventHandler_(DGWebViewEventTypeMessageHandler, message);
}
- (void)observeValueForKeyPath:(NSString *)keyPath ofObject:(id)object change:(NSDictionary<NSKeyValueChangeKey, id> *)change context:(void *)context {
    if ([keyPath isEqualToString: @"title"]) {
        eventHandler_(DGWebViewEventTypeTitleChanged, change[NSKeyValueChangeNewKey]);
    }
}

@end

@interface DGWebViewController: NSObject <WKUIDelegate>
@end

@implementation DGWebViewController {
	WKWebView* webView_;
    DGWebViewHandler* handler_;
    NSString* initErrorMessage_;
}

-(id)initWithParentView: (NSView*)parentView eventHandler: (DGWebViewEventHandler)eventHandler interceptedSchemes:(NSArray<NSString*>*)schemes message_handler_names:(NSArray<NSString*>*)names {
    self = [super init];
    if (self) {
        if (parentView.subviews.count > 0) {
            NSLog(@"parentView.subviews %@", parentView.subviews);
            initErrorMessage_ = @"DGWebViewController: parentView is not empty (subviews.count > 0)";
            return self;
        }

        handler_ = [[DGWebViewHandler alloc] initWithEventHandler:eventHandler];

        WKWebViewConfiguration* configuration = [[WKWebViewConfiguration alloc] init];

        @try {
            for (NSString* scheme in schemes) {
        	   [configuration setURLSchemeHandler: handler_ forURLScheme: scheme];
            }
        }
        @catch (NSException* exception) {
            initErrorMessage_ = [exception description];
            return self;
        }


        for (NSString* name in names) {
            [configuration.userContentController
                addScriptMessageHandler: handler_
                name: name
            ];
        }

    	webView_ = [[WKWebView alloc] initWithFrame: parentView.bounds configuration: configuration];
        webView_.autoresizingMask = NSViewWidthSizable | NSViewHeightSizable;
        [parentView addSubview: webView_];
        [webView_ setValue: @NO forKey:@"drawsBackground"];

		[webView_ setUIDelegate: handler_];
		[webView_ addObserver: handler_ forKeyPath: @"title" options: NSKeyValueObservingOptionNew context: nil];
    }
    return self;
}

-(NSString*)errorMessage {
    return initErrorMessage_;
}

-(void)addPreloadScript: (NSString*)script {
    [webView_.configuration.userContentController
        addUserScript: [[WKUserScript alloc]
            initWithSource: script
            injectionTime: WKUserScriptInjectionTimeAtDocumentStart
            forMainFrameOnly: NO
        ]
    ];
}

-(void)loadURL:(NSURL*)url {
    // NSLog(@"loadURL %@", initErrorMessage_);
    // [webView_ loadHTMLString: @"<b>hello!</b>" baseURL: nil];
    [webView_ loadRequest: [NSURLRequest requestWithURL: url]];
}


-(void)setDevToolsEnabled:(BOOL)enabled {
    [webView_.configuration.preferences setValue:@(enabled) forKey:@"developerExtrasEnabled"];
}

-(void)executeScript:(NSString*)script {
    [webView_ evaluateJavaScript: script completionHandler: nil];
}

@end

id DGWebViewControllerNew(NSView* parentView, DGWebViewEventHandler handler, NSArray<NSString*>* interceptedSchemes, NSArray<NSString*>* message_handler_names) {
    return [[DGWebViewController alloc]
    	initWithParentView: parentView
        eventHandler: handler
    	interceptedSchemes: interceptedSchemes
        message_handler_names: message_handler_names
    ];
}

void DGWKURLSchemeTaskSetResponse(id<WKURLSchemeTask> task, NSString* contentType, NSData* responseData) {
    [task didReceiveResponse: [[NSHTTPURLResponse alloc]
        // TODO: check if WKURLSchemeTask.request.URL is guaranteed non-null.
        initWithURL: task.request.URL
        statusCode: 200
        HTTPVersion: @"HTTP/1.1"
        headerFields: @{
            @"Content-Type": contentType,
            @"Content-Length": @([responseData length]).stringValue
        }
    ]];
    [task didReceiveData: responseData];
    [task didFinish];
}

