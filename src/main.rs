slint::include_modules!();
use slint::{LogicalPosition, SharedString};

fn main() {
    let ui = MainWindow::new();
    ui.window()
        .set_position(LogicalPosition { x: -3.0, y: -30.0 });

    let ui_handle = ui.as_weak();
    // ui.on_request_increase_value(move || {
    //     let ui = ui_handle.unwrap();
    //     ui.set_counter(ui.get_counter() + 1);
    // });
    ui_handle.unwrap().set_time(SharedString::from(format!(
        "{}",
        chrono::Local::now().format("%H:%M:%S")
    )));
    ui.run();
}
