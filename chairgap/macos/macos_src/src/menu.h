#ifndef DG_MENU_H
#define DG_MENU_H

#include <stdbool.h>

void DGMenuItemSetActionBlock(id menuItem, void(^block)());
void DGMenuItemSetActionSelector(id menuItem, SEL action);
void DGMenuItemSetEnabled(id menuItem, bool enabled);

void DGMenuItemSetCheckable(id menuItem, bool checkable);
bool DGMenuItemGetCheckable(id menuItem);

#endif
