slint::include_modules!();
use slint::{LogicalPosition, SharedString, Timer, TimerMode};
use sysinfo::{System, SystemExt};
mod utils;

struct SystemInfo {
    cpu_avg: f32,
    ram_percent: f32,
    network_down: i32,
    network_up: i32,
    temp: i32,
}

fn get_all_sys_info(req_sys: &mut sysinfo::System) -> SystemInfo {
    req_sys.refresh_all();

    let cpu_avg = utils::get_cpu_use(req_sys);
    let ram_percent = utils::get_ram_use(req_sys);
    let network_down = utils::get_ntwk_dwn(req_sys);
    let network_up = utils::get_ntwk_up(req_sys);
    let temp = utils::get_temp(req_sys);

    SystemInfo {
        cpu_avg,
        ram_percent,
        network_down,
        network_up,
        temp,
    }
}

fn main() {
    let mut current_sys = System::new_all();

    let ui = MainWindow::new();
    ui.window()
        .set_position(LogicalPosition { x: 80.0, y: -40.0 });

    let ui_handle = ui.as_weak();
    // ui.on_request_increase_value(move || {
    //     let ui = ui_handle.unwrap();
    //     ui.set_counter(ui.get_counter() + 1);
    // });
    let timer = Timer::default();
    timer.start(
        TimerMode::Repeated,
        std::time::Duration::from_millis(1000),
        move || {
            let system_info = get_all_sys_info(&mut current_sys);
            ui_handle
                .unwrap()
                .set_cpu(SharedString::from(format!("{:.2}", system_info.cpu_avg)));
            ui_handle.unwrap().set_ram(SharedString::from(format!(
                "{:.2}",
                system_info.ram_percent
            )));
            ui_handle
                .unwrap()
                .set_downdata(SharedString::from(format!("{}", system_info.network_down)));
            ui_handle
                .unwrap()
                .set_updata(SharedString::from(format!("{}", system_info.network_up)));
        },
    );
    let ui_handle = ui.as_weak();
    let timetimer = Timer::default();
    timetimer.start(
        TimerMode::Repeated,
        std::time::Duration::from_millis(100),
        move || {
            ui_handle.unwrap().set_time(SharedString::from(format!(
                "{}",
                chrono::Local::now().format("%H:%M:%S")
            )));
        },
    );
    ui.run();
}
