#[macro_use]
extern crate clap;
extern crate libuhyve;

#[cfg(feature = "yaml")]
use clap::{App, AppSettings, Arg, SubCommand};

fn main() {
    let yml = load_yaml!("cl_arguments.yml");
    let app = clap::App::from_yaml(yml).setting(clap::AppSettings::SubcommandRequired);
    let matches = app.get_matches();

    let status_code = match matches.subcommand() {
        ("start", Some(start_matches)) => {
            let path_to_app = start_matches.value_of("path").unwrap();
            match libuhyve::start_app(path_to_app) {
                Ok(status_code) => status_code,
                Err(status) => {
                    println!(
                        "[ERROR] Could not start the application (code: {}). Abort!",
                        status.code
                    );
                    std::process::exit(-1)
                }
            }
        }
        ("checkpoint", Some(chkpt_matches)) => {
            let path_to_checkpoint = chkpt_matches.value_of("path").unwrap();
            let full_checkpoint = if chkpt_matches.is_present("full-checkpoint") {
                true
            } else {
                false
            };
            match libuhyve::create_checkpoint(path_to_checkpoint, full_checkpoint) {
                Ok(status_code) => status_code,
                Err(status) => {
                    println!(
                        "[ERROR] Could not create the checkpoint (code: {}). Abort!",
                        status.code
                    );
                    std::process::exit(-1)
                }
            }
        }
        ("restore", Some(restore_matches)) => {
            let path_to_checkpoint = restore_matches.value_of("path").unwrap();
            match libuhyve::load_checkpoint(path_to_checkpoint) {
                Ok(status_code) => status_code,
                Err(status) => {
                    println!(
                        "[ERROR] Could not restore the checkpoint (code: {}). Abort!",
                        status.code
                    );
                    std::process::exit(-1)
                }
            }
        }
        ("migrate", Some(migrate_matches)) => {
            let destination = migrate_matches.value_of("destination").unwrap();
            let mig_type = migrate_matches.value_of("type").unwrap();
            let mig_mode = migrate_matches.value_of("mode").unwrap();
            let use_odp = if migrate_matches.is_present("use-odp") {
                true
            } else {
                false
            };
            let prefetch = if migrate_matches.is_present("prefetch") {
                true
            } else {
                false
            };
            match libuhyve::migrate(destination, mig_type, mig_mode, use_odp, prefetch) {
                Ok(status_code) => status_code,
                Err(status) => {
                    println!("[ERROR] Could not migrate (code: {}). Abort!", status.code);
                    std::process::exit(-1)
                }
            }
        }
        _ => unreachable!(),
    };

    println!("[INFO] Server returned: {}", status_code);
}
