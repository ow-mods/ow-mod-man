use std::fs::{create_dir_all, File};
use std::io::Write;

use anyhow::Result;
use clap::CommandFactory;
use clap_mangen::Man;

use crate::{get_out_dir, get_pkg_version};

include!("../../owmods_cli/src/cli.rs");

pub fn generate_man_files() -> Result<()> {
    let out_dir = get_out_dir()?.join("cli").join("man");
    create_dir_all(&out_dir)?;
    let man = Man::new(BaseCli::command());
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;
    std::fs::write(out_dir.join("owmods.1"), buffer)?;
    let cmd = BaseCli::command();
    for subcommand in cmd.get_subcommands() {
        let subcommand = subcommand.clone();
        let subcommand_name = format!("owmods-{}", subcommand.get_name());
        let mut buffer: Vec<u8> = Default::default();
        let man = Man::new(subcommand.clone().name(&subcommand_name));
        man.render(&mut buffer)?;
        std::fs::write(
            std::path::PathBuf::from(&out_dir).join(format!("{}.1", &subcommand_name)),
            buffer,
        )?;
    }
    Ok(())
}

pub fn generate_completions() -> Result<()> {
    let out_dir = get_out_dir()?.join("cli").join("completions");
    create_dir_all(&out_dir)?;
    let shells: Vec<Shell> = vec![Shell::Zsh, Shell::Bash, Shell::Fish];
    for shell in shells {
        let mut cmd = BaseCli::command();
        let shell_name = format!("{:?}", shell).to_ascii_lowercase();
        let mut file = File::create(out_dir.join(format!("owmods.{}", shell_name)))?;
        clap_complete::generate(shell, &mut cmd, "owmods", &mut file);
    }
    Ok(())
}

pub fn generate_cli_pkg_build() -> Result<()> {
    let version = get_pkg_version(include_str!("../../owmods_cli/Cargo.toml"));
    let pkgbuild = include_str!("cli_templates/PKGBUILD").replace("~~VERSION~~", version);
    let out_dir = get_out_dir()?.join("cli").join("pkgbuild");
    create_dir_all(&out_dir)?;
    let mut file = File::create(out_dir.join("PKGBUILD"))?;
    write!(file, "{}", pkgbuild)?;
    Ok(())
}
