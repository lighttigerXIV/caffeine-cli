use std::{
    fs,
    path::PathBuf,
    process::{exit, Command},
};

use clap::{Parser, ValueEnum};
use inquire::Select;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    pub command: Option<CaffeineCommand>,
}

#[derive(ValueEnum, Clone)]
enum CaffeineCommand {
    Enable,
    Disable,
}

fn main() {
    let args = Args::parse();

    if let Some(command) = args.command {
        match command {
            CaffeineCommand::Enable => enable_caffeine(),
            CaffeineCommand::Disable => disable_caffeine(),
        }
    }

    let enable_option = "Enable Caffeine Session";
    let disable_option = "Disable Caffeine Session";
    let close_option = "Close";
    let options = vec![enable_option, disable_option, close_option];

    let answer = Select::new("Select one of the options:", options).prompt();

    if let Ok(answer) = answer {
        if answer == enable_option {
            enable_caffeine();
        } else if answer == disable_option {
            disable_caffeine();
        } else {
            exit(0)
        }
    }
}

fn get_id_path() -> PathBuf {
    PathBuf::from("/tmp/caffeine-id.txt")
}

fn get_id() -> Option<String> {
    let cookie = fs::read_to_string(get_id_path());

    if let Ok(cookie) = cookie {
        return Some(cookie);
    } else {
        return None;
    }
}

fn enable_caffeine() {
    let id = get_id();

    if id.is_some() {
        println!("🔴 Caffeine is currently enabled");
        exit(1);
    }

    let child = Command::new("systemd-inhibit")
        .arg("--what=idle")
        .arg("sleep")
        .arg("infinity")
        .spawn()
        .expect("Error running caffeine command");

    let process_id = child.id();

    fs::write(get_id_path(), process_id.to_string()).expect("Error writing id");

    println!("☕ Caffeine session enabled");

    exit(0);
}

fn disable_caffeine() {
    let id = get_id();

    if let Some(id) = id {
        Command::new("kill")
            .arg(&id)
            .spawn()
            .expect("Error killing caffeine session");

        fs::remove_file(get_id_path()).expect("Error removing id file");

        println!("😴 Caffeine session disabled");
        exit(0);
    }

    println!("🔴 There's no caffeine session enabled");
    exit(1);
}
