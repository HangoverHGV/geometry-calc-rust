use slint::ComponentHandle;

use super::schedule_shutdown::{cancel, restore_remaining, schedule};
use crate::{AppWindow, Backend};

const MSG_INFO: i32 = 0;
const MSG_ERROR: i32 = 2;

pub fn setup_callbacks(ui: &AppWindow) {
    // Pick up a shutdown scheduled before the app was last closed.
    if let Some(secs) = restore_remaining() {
        ui.global::<Backend>().set_shutdown_remaining(secs);
    }

    let weak = ui.as_weak();
    ui.global::<Backend>().on_schedule_shutdown(move |secs| {
        let Some(ui) = weak.upgrade() else { return };
        match schedule(secs) {
            Ok(msg) => {
                ui.global::<Backend>().set_shutdown_remaining(secs);
                ui.invoke_trigger_backend_message(MSG_INFO, msg.into());
            }
            Err(msg) => ui.invoke_trigger_backend_message(MSG_ERROR, msg.into()),
        }
    });

    let weak = ui.as_weak();
    ui.global::<Backend>().on_cancel_shutdown(move || {
        let Some(ui) = weak.upgrade() else { return };
        match cancel() {
            Ok(msg) => {
                ui.global::<Backend>().set_shutdown_remaining(0);
                ui.invoke_trigger_backend_message(MSG_INFO, msg.into());
            }
            Err(msg) => ui.invoke_trigger_backend_message(MSG_ERROR, msg.into()),
        }
    });
}