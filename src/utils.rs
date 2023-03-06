use sysinfo::{ComponentExt, CpuExt, NetworkExt, SystemExt};

// Get the average core usage
pub fn get_cpu_use(req_sys: &sysinfo::System) -> f32 {
    // Put all of the core loads into a vector
    let mut cpus: Vec<f32> = Vec::new();
    for core in req_sys.cpus() {
        cpus.push(core.cpu_usage());
    }

    // Get the average load
    let cpu_tot: f32 = cpus.iter().sum();
    let cpu_avg: f32 = cpu_tot / cpus.len() as f32;

    cpu_avg
}

// Divide the used RAM by the total RAM
pub fn get_ram_use(req_sys: &sysinfo::System) -> f32 {
    (req_sys.used_memory() as f32) / (req_sys.total_memory() as f32) * 100.
}

// Get the total network (down) usage
pub fn get_ntwk_dwn(req_sys: &sysinfo::System) -> i32 {
    // Get the total bytes recieved by every network interface
    let mut rcv_tot: Vec<i32> = Vec::new();
    for (_interface_name, ntwk) in req_sys.networks() {
        rcv_tot.push(ntwk.received() as i32);
    }

    // Total them and convert the bytes to KB
    let ntwk_tot: i32 = rcv_tot.iter().sum();

    ntwk_tot / 128
}

// Get the total network (up) usage
pub fn get_ntwk_up(req_sys: &sysinfo::System) -> i32 {
    // Get the total bytes sent by every network interface
    let mut snd_tot: Vec<i32> = Vec::new();
    for (_interface_name, ntwk) in req_sys.networks() {
        snd_tot.push(ntwk.transmitted() as i32);
    }

    // Total them and convert the bytes to KB
    let ntwk_tot: i32 = snd_tot.iter().sum();

    ntwk_tot / 128
}

// Get the temperature of the CPU
pub fn get_temp(req_sys: &sysinfo::System) -> i32 {
    // For every component, if it's the CPU, put its temperature in variable to return
    let mut wanted_temp: f32 = -1.;
    for comp in req_sys.components() {
        if comp.label() == "CPU" {
            wanted_temp = comp.temperature();
        }
    }

    wanted_temp as i32
}
