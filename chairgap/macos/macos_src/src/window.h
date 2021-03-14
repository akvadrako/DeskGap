#ifndef DG_WINDOW
#define DG_WINDOW

#include "geo.h"

typedef enum {
    DGWindowEventTypeBlur,
    DGWindowEventTypeFocus,
    DGWindowEventTypeMove,
    DGWindowEventTypeClose,
    DGWindowEventTypeResize
} DGWindowEventType;

typedef enum {
    DGWindowTitleBarStyleDefault,
    DGWindowTitleBarStyleHidden
} DGWindowTitleBarStyle;

typedef enum {
    DGWindowTrafficLightStyleDefault,
    DGWindowTrafficLightStyleCustom,
    DGWindowTrafficLightStyleHidden
} DGWindowTrafficLightStyle;

// typedef struct {
// 	double value;
// 	DGLayoutUnit unit;
// } DGLayoutQuantity;

// typedef enum {
// 	DGLayoutUnitPoint,
// 	DGLayoutUnitPercentage
// } DGLayoutUnit;

// typedef enum {
// 	DGLayoutAnchorStart,
// 	DGLayoutAnchorEnd,
// 	DGLayoutAnchorEnter
// } DGLayoutAnchor;

// typedef struct {
// 	DGLayoutQuantity offset;
// 	DGLayoutAnchor anchor;
// } DGAxisPosition;

// typedef struct {
// 	DGLayoutQuantity width;
// 	DGLayoutQuantity height;
// 	DGAxisPosition horizontal_position;
// 	DGAxisPosition vertical_position;
// } DGLayout;

typedef struct {
	int32_t size;
	//TODO: material
} DGVisualEffectViewConfig;

typedef void (^DGWindowEventHandler)(DGWindowEventType);

id DGWindowControllerNew(DGWindowEventHandler handler);

#endif
