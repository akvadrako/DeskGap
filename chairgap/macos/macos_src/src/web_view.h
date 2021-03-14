#ifndef DG_WEB_VIEW
#define DG_WEB_VIEW


typedef enum {
    DGWebViewEventTypeTitleChanged,
    DGWebViewEventTypeStartURLSchemeTask,
    DGWebViewEventTypeEndURLSchemeTask,
    DGWebViewEventTypeMessageHandler,
} DGWebViewEventType;

typedef void (^DGWebViewEventHandler)(DGWebViewEventType, id);

id DGWebViewControllerNew(id contentView, DGWebViewEventHandler handler, id interceptedSchemes, id messageHandlerNames);

void DGWKURLSchemeTaskSetResponse(id task, id contentType, id responseData);

#endif
