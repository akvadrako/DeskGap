#ifndef DG_APP
#define DG_APP

typedef enum { 
    DGAppEventTypeWillLaunch,
    DGAppEventTypeShouldClose,
    DGAppEventTypeReopen,
} DGAppEventType;

typedef void (^DGAppEventHandler)(DGAppEventType, void*);

id DGAppDelegateNew(DGAppEventHandler event_handler);
void DGAppTick();

#endif
