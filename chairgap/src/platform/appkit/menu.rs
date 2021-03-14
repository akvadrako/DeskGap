use super::super::common::menu::MenuItemTemplate;
use super::strings::ns_string;
use crate::platform::common::menu::{
    MacOSSpecialMenuType, MenuItemAction, MenuItemStyle, MenuTemplate, MenuUpdateError,
};
use block::{Block, ConcreteBlock};
use cocoa::appkit::{NSApp, NSApplication, NSMenu, NSMenuItem};
use cocoa::base::{id, nil};
use cocoa::foundation::NSInteger;
use objc::rc::StrongPtr;
use objc::runtime::{Object, Sel, NO, YES};
use objc::*;
use serde::export::PhantomData;
use std::rc::Rc;

const NS_CONTROL_STATE_VALUE_MIXED: NSInteger = -1;
const NS_CONTROL_STATE_VALUE_OFF: NSInteger = 0;
const NS_CONTROL_STATE_VALUE_ON: NSInteger = 1;

pub fn sel_miniaturize() -> Sel {
    sel!(miniaturize:)
}
pub fn sel_close() -> Sel {
    sel!(close:)
}

pub(super) fn ns_menu(tmpl: &MenuTemplate, title: &str) -> StrongPtr {
    unsafe {
        let title = ns_string(title);
        let ns_menu = StrongPtr::new(NSMenu::initWithTitle_(NSMenu::alloc(nil), *title));
        for item_tmpl in &tmpl.items {
            let ns_menu_item = ns_menu_item(item_tmpl);
            NSMenu::addItem_(*ns_menu, *ns_menu_item);
        }
        if let Some(menu_type) = &tmpl.macos_special_menu_type {
            match menu_type {
                MacOSSpecialMenuType::Services => {
                    cocoa::appkit::NSApplication::setServicesMenu_(NSApp(), *ns_menu)
                }
                MacOSSpecialMenuType::Window => {
                    cocoa::appkit::NSApplication::setWindowsMenu_(NSApp(), *ns_menu)
                }
                MacOSSpecialMenuType::Help => {}
            }
        }
        ns_menu
    }
}

pub(super) fn ns_menu_item(tmpl: &MenuItemTemplate) -> StrongPtr {
    unsafe {
        match tmpl {
            MenuItemTemplate::Separator => StrongPtr::retain(NSMenuItem::separatorItem(nil)),
            MenuItemTemplate::Label(labelTmpl) => {
                let item = StrongPtr::new(NSMenuItem::new(nil));

                let ns_label = ns_string(labelTmpl.label.as_str());
                let _: () = msg_send![*item, setTitle: *ns_label];

                match &labelTmpl.style {
                    MenuItemStyle::Normal => {
                        chairgap_sys::DGMenuItemSetCheckable(*item, false);
                    }
                    MenuItemStyle::Checkable { checked, .. } => {
                        chairgap_sys::DGMenuItemSetCheckable(*item, true);
                        let _: () = msg_send![*item, setState: if *checked { NS_CONTROL_STATE_VALUE_ON } else { NS_CONTROL_STATE_VALUE_OFF } ];
                    }
                }

                match &labelTmpl.action {
                    MenuItemAction::Fn(on_click_fn) => {
                        let on_click_fn = Rc::clone(on_click_fn);
                        let block = ConcreteBlock::new(move || on_click_fn());
                        let block = block.copy();
                        let block: &Block<(), ()> = &block;
                        chairgap_sys::DGMenuItemSetActionBlock(*item, block);
                    }
                    MenuItemAction::Submenu(menu_tmpl) => {
                        let submenu = ns_menu(menu_tmpl, labelTmpl.label.as_str());
                        NSMenuItem::setSubmenu_(*item, *submenu);
                    }
                    MenuItemAction::MacOSSelector(action) => {
                        chairgap_sys::DGMenuItemSetActionSelector(*item, action.clone());
                    }
                };

                chairgap_sys::DGMenuItemSetEnabled(*item, labelTmpl.enabled);
                item
            }
        }
    }
}

fn get_nth_item(ns_menu: id, index: u32) -> Option<id> {
    let index = index as NSInteger;
    let item_count: NSInteger = unsafe { msg_send![ns_menu, numberOfItems] };
    if index + 1 > item_count {
        None
    } else {
        Some(unsafe { msg_send![ns_menu, itemAtIndex: index] })
    }
}

fn get_item_by_path<'a>(
    ns_menu: id,
    menu_item_path: impl IntoIterator<Item = &'a u32>,
) -> Option<id> {
    let mut iter = menu_item_path.into_iter();
    let first_index = iter.next()?;
    let mut current_item_ptr = get_nth_item(ns_menu, *first_index)?;
    for index in iter {
        let sub_menu_ptr: id = unsafe { msg_send![current_item_ptr, submenu] };
        if sub_menu_ptr == nil {
            return None;
        }
        current_item_ptr = get_nth_item(sub_menu_ptr, *index)?;
    }
    Some(current_item_ptr)
}

pub(super) struct Menu<'a>(StrongPtr, PhantomData<&'a ()>);
impl Menu<'_> {
    pub(super) fn set_enabled<'a>(
        &'a self,
        menu_item_path: impl IntoIterator<Item = &'a u32>,
        enabled: bool,
    ) -> Result<(), MenuUpdateError> {
        match get_item_by_path(*self.0, menu_item_path) {
            None => Err(MenuUpdateError::MenuItemNotFound),
            Some(item_ptr) => {
                if unsafe { msg_send![item_ptr, isSeparatorItem] } {
                    Err(MenuUpdateError::InvalidMenuItemType)
                } else {
                    unsafe { chairgap_sys::DGMenuItemSetEnabled(item_ptr, enabled) };
                    Ok(())
                }
            }
        }
    }

    pub(super) fn set_checked<'a>(
        &'a self,
        menu_item_path: impl IntoIterator<Item = &'a u32>,
        checked: bool,
    ) -> Result<(), MenuUpdateError> {
        match get_item_by_path(*self.0, menu_item_path) {
            None => Err(MenuUpdateError::MenuItemNotFound),
            Some(item_ptr) => {
                if unsafe {
                    msg_send![item_ptr, isSeparatorItem]
                        || !chairgap_sys::DGMenuItemGetCheckable(item_ptr)
                } {
                    Err(MenuUpdateError::InvalidMenuItemType)
                } else {
                    let _: () = unsafe {
                        msg_send![item_ptr, setState: if checked { NS_CONTROL_STATE_VALUE_ON } else { NS_CONTROL_STATE_VALUE_OFF } ]
                    };
                    Ok(())
                }
            }
        }
    }
}
