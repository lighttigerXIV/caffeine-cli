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

    let enable_option = "Enable Caffeine";
    let disable_option = "Disable Caffeine";
    let cancel_option = "Cancel";
    let options = vec![enable_option, disable_option, cancel_option];

    let answer = Select::new("Select one of the following options", options).prompt();

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

fn get_cookie_path() -> PathBuf {
    PathBuf::from("/tmp/caffeine-cookie.txt")
}

fn get_cookie() -> Option<String> {
    let cookie = fs::read_to_string(get_cookie_path());

    if let Ok(cookie) = cookie {
        return Some(cookie);
    } else {
        return None;
    }
}

fn enable_caffeine() {
    let cookie = get_cookie();

    if cookie.is_some() {
        println!("☕ A caffeine session is already active");
        exit(0)
    }

    let command = Command::new("sh")
        .arg("-c")
        .arg(r#"dbus-send --session --dest=org.freedesktop.ScreenSaver --type=method_call --print-reply /org/freedesktop/ScreenSaver org.freedesktop.ScreenSaver.Inhibit string:"caffeine" string:"prevent lock screen""#)
        .output()
        .expect("⚠️ Error running dbus command");

    let command_output = String::from_utf8_lossy(&command.stdout);
    let output_split: Vec<&str> = command_output.split_whitespace().collect();

    if let Some(cookie_id) = output_split.last() {
        fs::write(get_cookie_path(), cookie_id).expect("Error writing cookie");

        println!("☕ You are now caffeinated");

        exit(0);
    }

    exit(1);
}

fn disable_caffeine() {
    let cookie = get_cookie();

    if let Some(cookie) = cookie {
        let command = format!(
            r#"dbus-send --session --dest=org.freedesktop.ScreenSaver --type=method_call --print-reply /org/freedesktop/ScreenSaver org.freedesktop.ScreenSaver.UnInhibit uint32:{}"#,
            cookie
        );

        let output = Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()
            .expect("Error running command");

        if output.status.success() {
            fs::remove_file(get_cookie_path()).expect("Error removing cookie");

            println!("☕ You are now caffeine deprived");

            exit(0);
        }
    } else {
        println!("☕ Could not find an active session");

        exit(1);
    }
}
