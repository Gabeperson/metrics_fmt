use sysinfo::{CpuExt, NetworkExt, NetworksExt, System, SystemExt};

#[derive(Debug, Default)]
pub struct SysinfoExporter {
    name: String,
    id: String,
    system: System,
}

impl SysinfoExporter {
    pub fn new(name: String, id: String) -> Self {
        let mut system = System::new_all();
        system.refresh_all();
        Self { system, name, id }
    }

    pub fn get(&mut self) -> String {
        self.system.refresh_cpu();
        self.system.refresh_memory();
        // self.system.refresh_disks();
        self.system.refresh_networks();

        let system = &self.system;
        let name = &self.name;
        let id = &self.id;

        let nameid = format!("{name}_{id}");
        let cpus = system
            .cpus()
            .iter()
            .enumerate()
            .map(|(index, cpu)| {
                let cpu_name = cpu.name();
                let usage = cpu.cpu_usage();
                format!("{nameid}_cpu_usage{{name=\"{cpu_name}\",id=\"{index}\"}} {usage:.8}\n")
            })
            .collect::<String>();

        let memory = {
            let total = system.total_memory();
            let used = system.used_memory();
            let used_percent = used as f32 / total as f32 * 100.;
            let total_swap = system.total_swap();
            let used_swap = system.used_swap();
            let swap_percent = used_swap as f32 / total_swap as f32 * 100.;
            format!(
                "\
                {nameid}_memory_total {total} \n\
                {nameid}_memory_used {used} \n\
                {nameid}_memory_percent {used_percent} \n\
                {nameid}_swap_total {total_swap} \n\
                {nameid}_swap_used {used_swap} \n\
                {nameid}_swap_percent {swap_percent}\n\
                "
            )
        };

        // doesn't work for some reason. Maybe i'll fix it sometime?
        // let disk = {
        //     let total = system
        //         .disks()
        //         .iter()
        //         .map(|disk| disk.total_space())
        //         .sum::<u64>();
        //     let avail = system
        //         .disks()
        //         .iter()
        //         .map(|disk| disk.total_space())
        //         .sum::<u64>();
        //     let used = total - avail;
        //     let used_percent = used as f32 / total as f32 * 100.;
        //     dbg!(&avail);
        //     dbg!(&used);
        //     dbg!(&system.disks());
        //     format!(
        //         "\
        //         {nameid}_disk_total {total} \n\
        //         {nameid}_disk_used {used} \n\
        //         {nameid}_disk_percent {used_percent} \n\
        //         "
        //     )
        // };
        let network = {
            let sent = system
                .networks()
                .iter()
                .map(|(_name, data)| data.total_transmitted())
                .sum::<u64>();
            let received = system
                .networks()
                .iter()
                .map(|(_name, data)| data.total_received())
                .sum::<u64>();

            format!(
                "\
                {nameid}_network_sent {sent} \n\
                {nameid}_network_received {received} \n\
                "
            )
        };
        format!("{cpus}\n{memory}\n{network}")
    }
}
