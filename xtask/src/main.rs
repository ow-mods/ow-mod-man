use std::path::PathBuf;

use anyhow::Result;
use cli_tasks::{generate_cli_pkg_build, generate_completions, generate_man_files, print_version};
use gui_tasks::generate_gui_pkg_build;
use gui_disable_updater::disable_updater;
use regex::RegexBuilder;

mod cli_tasks;
mod gui_tasks;
mod gui_disable_updater;
mod log_client;
mod log_spammer;


pub fn get_out_dir() -> Result<PathBuf> {
    let out_dir = std::path::PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap())
        .parent()
        .unwrap()
        .join("dist");
    Ok(out_dir)
}

pub fn get_pkg_version(in_str: &str) -> &str {
    let re = RegexBuilder::new(r#"^version = "(.*?)"$"#)
        .multi_line(true)
        .build()
        .unwrap();
    let group = re.captures_iter(in_str).next().unwrap();
    let version_match = group.get(1).unwrap();
    version_match.as_str()
}

fn main() -> Result<()> {
    let cmd = std::env::args().nth(1).expect("Missing Command Name");
    match cmd.as_str() {
        "gen_man" => generate_man_files()?,
        "gen_completions" => generate_completions()?,
        "dist_cli" => {
            println!("Generating Completions...");
            generate_completions()?;
            println!("Generating Man Pages...");
            generate_man_files()?;
        }
        "cli_pkg_build" => generate_cli_pkg_build()?,
        "gui_pkg_build" => generate_gui_pkg_build()?,
        "gui_disable_updater" => disable_updater()?,
        "cli_version" => print_version()?,
        "log_client" => log_client::log_client()?,
        "spam_logs" => log_spammer::spam_logs(
            std::env::args()
                .nth(2)
                .expect("Enter port")
                .parse()
                .unwrap(),
        )?,
        _ => panic!("Invalid Command: {cmd}"),
    }
    Ok(())
}
