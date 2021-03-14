use crate::platform::common::menu::{
    LabelMenuItemTemplate, MenuItemAction, MenuItemStyle, MenuItemTemplate,
};
use std::rc::Rc;

pub struct MenuItem(MenuItemTemplate);
impl MenuItem {
    pub fn separator() -> SeparatorMenuItem {
        SeparatorMenuItem {}
    }
    pub fn label(title: &str) -> LabelMenuItem {
        LabelMenuItem {
            0: LabelMenuItemTemplate {
                label: title.to_string(),
                enabled: false,
                style: MenuItemStyle::Normal,
                action: MenuItemAction::Fn(Rc::new(|| {})),
            },
        }
    }
}

pub struct SeparatorMenuItem;
impl From<SeparatorMenuItem> for MenuItem {
    fn from(_: SeparatorMenuItem) -> Self {
        MenuItem(MenuItemTemplate::Separator)
    }
}

pub struct LabelMenuItem(LabelMenuItemTemplate);
impl From<LabelMenuItem> for MenuItem {
    fn from(item: LabelMenuItem) -> Self {
        MenuItem(MenuItemTemplate::Label(item.0))
    }
}
impl LabelMenuItem {}
