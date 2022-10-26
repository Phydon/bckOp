use std::{
    error::Error,
    process::{self, Command},
};

fn main() {
    let commands = vec![
        // load custom commands in nushell
        "use ~/AppData/Roaming/nushell/init.nu *",
        // make obsidian backup
        "mkdir ~/backup/bckOp",
        "let date = (date now | date format %d_%m_%Y_%H_%M_%S)",
        "cp -r ~/obsidian_vault/ $\"C:/Users/Pohl/backup/bckOp/obsidian_vault_($date)\"",
    ];

    let args: String = collect_args(commands).expect("Unable to collect args");

    if let Err(err) = cmd(args.as_str()) {
        println!("Error executing cmd: {}", err);
        process::exit(1);
    }
}

fn cmd(args: &str) -> Result<(), Box<dyn Error>> {
    if cfg!(target_os = "windows") {
        Command::new("C:/Program Files/nu/bin/nu.exe")
            .args(["-c", args])
            .status()?
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("echo 'not implemented yet'")
            .status()?
    };

    Ok(())
}

fn collect_args(args_list: Vec<&str>) -> Result<String, Box<dyn Error>> {
    let mut combiner = String::new();
    for arg in args_list {
        combiner.push_str(arg);
        combiner.push_str("; ");
    }

    Ok(combiner)
}
