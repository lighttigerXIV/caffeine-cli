use std::process::exit;

use caffeine::{end_protected_session, get_session, init_protected_session, CaffeineSession};
use clap::{Parser, Subcommand};
use inquire::{CustomType, Select};

pub mod caffeine;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    pub commands: Option<ArgCommand>,
}

#[derive(Subcommand, Debug)]
pub enum ArgCommand {
    #[command(about = "Enables the caffeine session with an infinnite amount of time")]
    Enable,

    #[command(about = "Enables the caffeine session with x amount of minutes")]
    Timed {
        #[arg(short, long, help = "The amount of minutes for the session")]
        minutes: u64,
    },

    #[command(about = "Disables the current session")]
    Disable,

    #[command(about = "Shows the status of the current session")]
    Status,
}

fn main() {
    let args = Args::parse();

    match args.commands {
        Some(command) => match command {
            ArgCommand::Enable => {
                enable_caffeine(None);
            }
            ArgCommand::Timed { minutes } => {
                enable_caffeine(Some(minutes * 60));
            }
            ArgCommand::Disable => {
                disable_caffeine();
            }
            ArgCommand::Status => {
                show_status();
            }
        },
        None => {}
    }

    let enable_option = "☕ Enable Caffeine";
    let enable_timed_option = "☕ Enable Timed Caffeine";
    let disable_option = "😴 Disable Caffeine";
    let status_option = "🗒️  Get Session Status";
    let close_option = "🚪 Close";

    let options = vec![
        enable_option,
        enable_timed_option,
        disable_option,
        status_option,
        close_option,
    ];

    let answer = Select::new("Select one of the options:", options).prompt();

    if let Ok(answer) = answer {
        if answer == enable_option {
            enable_caffeine(None);
        } else if answer == enable_timed_option {
            let five_mins_option = "5 minutes";
            let ten_mins_option = "10 minutes";
            let fifteen_mins_option = "15 minutes";
            let twenty_mins_option = "20 minutes";
            let thirty_mins_option = "30 minutes";
            let one_hour_option = "1 hour";
            let two_hours_option = "2 hours";
            let other_option = "Other";

            let options = vec![
                five_mins_option,
                ten_mins_option,
                fifteen_mins_option,
                twenty_mins_option,
                thirty_mins_option,
                one_hour_option,
                two_hours_option,
                other_option,
            ];

            let answer = Select::new("Select a time:", options).prompt();

            if let Ok(answer) = answer {
                if answer == five_mins_option {
                    enable_caffeine(Some(5 * 60));
                } else if answer == ten_mins_option {
                    enable_caffeine(Some(10 * 60));
                } else if answer == fifteen_mins_option {
                    enable_caffeine(Some(15 * 60));
                } else if answer == twenty_mins_option {
                    enable_caffeine(Some(20 * 60));
                } else if answer == thirty_mins_option {
                    enable_caffeine(Some(30 * 60));
                } else if answer == one_hour_option {
                    enable_caffeine(Some(60 * 60));
                } else if answer == two_hours_option {
                    enable_caffeine(Some(120 * 60));
                } else {
                    let minutes = CustomType::<u64>::new("Write the time in minutes:")
                        .with_error_message("Please type a valid number")
                        .prompt();

                    match minutes {
                        Ok(minutes) => {
                            enable_caffeine(Some(minutes * 60));
                        }
                        Err(_) => println!("Error getting number"),
                    }
                }
            }
        } else if answer == disable_option {
            disable_caffeine();
        } else if answer == status_option {
            show_status();
        } else {
            exit(0)
        }
    }
}

fn enable_caffeine(seconds: Option<u64>) {
    let session = init_protected_session(seconds);

    match session {
        Ok(_session) => {
            println!("☕ Caffeine session enabled");
            exit(0);
        }
        Err(e) => {
            println!("🐛 Error: {}", e);
            exit(1);
        }
    }
}

fn disable_caffeine() {
    let session = get_session();

    if session.is_none() {
        println!("🔴 Caffeine session is not active");
        exit(0);
    }

    match end_protected_session() {
        Ok(_) => {
            println!("😴 Caffeine session disabled");
            exit(0);
        }
        Err(e) => {
            println!("🐛 Error: {}", e);
            exit(1);
        }
    }
}

fn show_status() {
    let session = get_session();

    if let Some(session) = session {
        println!("🟢 Caffeine session is active");
        println!("🕑 Elapsed time: {}", session.get_elapsed_time());

        if session.session_length.is_some() {
            println!(
                "🐌 Session Length: {}",
                session.get_session_length().unwrap()
            );
            println!(
                "⏲️  Remaining time: {}",
                session.get_remaining_time().unwrap()
            )
        }
    } else {
        println!("🔴 Caffeine session is not active");
    }

    exit(0);
}

impl CaffeineSession {}
