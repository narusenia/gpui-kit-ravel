use gpui::App;

mod app_menu_bar;
mod context_menu;
mod dropdown_menu;
mod menu_item;
mod popup_menu;

pub use app_menu_bar::{AppMenuBar, CONTEXT as APP_MENU_BAR_CONTEXT};
pub use context_menu::{ContextMenu, ContextMenuExt, ContextMenuState};
pub use dropdown_menu::DropdownMenu;
pub use popup_menu::{CONTEXT as POPUP_MENU_CONTEXT, PopupMenu, PopupMenuItem};

pub(crate) fn init(cx: &mut App) {
    app_menu_bar::init(cx);
    popup_menu::init(cx);
}
