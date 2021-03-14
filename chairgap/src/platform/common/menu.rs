use std::rc::Rc;

#[cfg(target_os = "macos")]
use objc::runtime::Sel;

#[cfg(target_os = "macos")]
pub(crate) enum MacOSSpecialMenuType {
    Services,
    Window,
    Help,
}

pub(crate) struct MenuTemplate {
    pub items: Vec<MenuItemTemplate>,
    #[cfg(target_os = "macos")]
    pub macos_special_menu_type: Option<MacOSSpecialMenuType>,
}

pub(crate) enum MenuItemTemplate {
    Separator,
    Label(LabelMenuItemTemplate),
}

pub(crate) enum MenuItemStyle {
    Normal,
    Checkable {
        style: MenuItemCheckStyle,
        checked: bool,
    },
}

pub(crate) enum MenuItemCheckStyle {
    CheckBox,
    Radio,
}

pub(crate) enum MenuItemAction {
    Fn(Rc<dyn Fn() + 'static>),
    Submenu(MenuTemplate),
    #[cfg(target_os = "macos")]
    MacOSSelector(Sel),
}

pub(crate) struct LabelMenuItemTemplate {
    pub label: String,
    pub enabled: bool,
    pub style: MenuItemStyle,
    pub action: MenuItemAction,
}

pub(crate) enum MenuUpdateError {
    MenuItemNotFound,
    InvalidMenuItemType,
}
