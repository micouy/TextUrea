#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]


#[cfg(target_os = "macos")]
use objc::{
	*,
    rc::StrongPtr,
    runtime::{Class, Object},
};

use tauri::{MenuEntry, Submenu, MenuItem, Menu, CustomMenuItem};

fn main() {
    const COPY_ITEM_ID: &'static str = "copy-menu-item";

    let copy_item = CustomMenuItem::new(COPY_ITEM_ID, "Copy")
        .accelerator("CommandOrCtrl+C");

    let txt_menu = Menu::new()
        .add_item(copy_item)
        .add_native_item(MenuItem::Paste)
        .add_native_item(MenuItem::Cut)
        .add_native_item(MenuItem::Undo)
        .add_native_item(MenuItem::Redo)
        .add_native_item(MenuItem::SelectAll);

    let menu = Menu::new()
        .add_submenu(Submenu::new("TextUrea", txt_menu));
	#[cfg(target_os = "macos")]
	let ns_app = unsafe {
    	let ns_app_cls = class!(NSApplication);
        let ns_app: &Object = msg_send![ns_app_cls, alloc];

		ns_app
	};

    tauri::Builder::default()
        .menu(menu)
        .on_menu_event(|event| {
			let id = event.menu_item_id();

            if id == COPY_ITEM_ID {
                event.window().emit("copy", ());

				#[cfg(target_os = "macos")]
				unsafe {
                    let sel = sel!(hide:);
                    assert!(ns_app.verify_message::<(&Object,), ()>(sel).is_ok());
					let _: () = ns_app.send_message(sel, ()).unwrap(); 
				}

				#[cfg(not(target_os = "macos"))]
				event.window().hide();
            }
        })
        .run(tauri::generate_context!())
        .expect("Unexpected error. Please file an issue.");
}
