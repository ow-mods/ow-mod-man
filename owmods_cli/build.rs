use std::fs::create_dir_all;

use clap::CommandFactory;
use clap_mangen::Man;

include!("src/cli.rs");

fn main() -> std::io::Result<()> {
    let out_dir = std::path::PathBuf::from(
        std::env::var_os("CARGO_MANIFEST_DIR").ok_or(std::io::ErrorKind::NotFound)?,
    )
    .join("manpage");
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
        let man = Man::new(subcommand.clone());
        man.render(&mut buffer)?;
        std::fs::write(
            std::path::PathBuf::from(&out_dir).join(format!("{}{}", &subcommand_name, ".1")),
            buffer,
        )?;
    }
    Ok(())
}
