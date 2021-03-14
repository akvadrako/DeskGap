#import <AppKit/AppKit.h>

#include "menu.h"

@interface DGMenuItemInfo: NSObject
@property (nonatomic) SEL globalAction;
@property (nonatomic) BOOL enabled;
@property (nonatomic) BOOL checkable;
@property (nonatomic, strong) void(^actionBlock)(id);
@end

@implementation DGMenuItemInfo

@synthesize globalAction;
@synthesize actionBlock;
@synthesize checkable;

-(void)callActionBlock:(id)sender {
	void(^block)() = [self actionBlock];
	if (block != nil) {
		block(sender);
	}
}
@end


static DGMenuItemInfo* getInfo(NSMenuItem* menuItem) {
	id existingInfo = [menuItem representedObject];
	if (existingInfo == nil) {
		DGMenuItemInfo* newInfo = [DGMenuItemInfo new];
		[menuItem setRepresentedObject: newInfo];
		return newInfo;
	}

	if (![existingInfo isMemberOfClass: [DGMenuItemInfo class]]) {
		fprintf(stderr, "%s\n", "menuItem.representedObject is not a DGMenuItemInfo");
		abort();
	}
	return existingInfo;
}

static void updateItem(NSMenuItem* menuItem) {
	DGMenuItemInfo* info = [menuItem representedObject];
	if (![info enabled]) {
		[menuItem setTarget: nil];
		[menuItem setAction: NULL];
	}
	else if ([info globalAction] != NULL) {
		[menuItem setTarget: nil];
		[menuItem setAction: [info globalAction]];
	}
	else {
		[menuItem setTarget: info];
		[menuItem setAction: @selector(callActionBlock:)];
	}
}

void DGMenuItemSetActionBlock(NSMenuItem* menuItem, void(^block)()) {
	DGMenuItemInfo* info = getInfo(menuItem);
	info.globalAction = NULL;
	info.actionBlock = block;
	updateItem(menuItem);
}

void DGMenuItemSetActionSelector(NSMenuItem* menuItem, SEL action) {
	DGMenuItemInfo* info = getInfo(menuItem);
	info.globalAction = action;
	info.actionBlock = nil;
	updateItem(menuItem);
}

void DGMenuItemSetEnabled(NSMenuItem* menuItem, bool enabled) {
	DGMenuItemInfo* info = getInfo(menuItem);
	info.enabled = enabled ? YES : NO;
	updateItem(menuItem);
}


void DGMenuItemSetCheckable(id menuItem, bool checkable) {
	DGMenuItemInfo* info = getInfo(menuItem);
	info.checkable = checkable ? YES : NO;
}

bool DGMenuItemGetCheckable(id menuItem) {
	DGMenuItemInfo* info = getInfo(menuItem);
	return info.checkable ? true : false;
}
