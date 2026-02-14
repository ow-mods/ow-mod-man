use std::fs::{create_dir_all, File};
use std::io::Write;

use anyhow::Result;

use crate::{get_out_dir, get_pkg_version};

pub fn generate_gui_pkg_build() -> Result<()> {
    let version = get_pkg_version(include_str!("../../owmods_gui/backend/Cargo.toml"));
    let pkgbuild = include_str!("gui_templates/PKGBUILD").replace("~~VERSION~~", version);
    let out_dir = get_out_dir()?.join("gui").join("pkgbuild");
    create_dir_all(&out_dir)?;
    let mut file = File::create(out_dir.join("PKGBUILD"))?;
    write!(file, "{pkgbuild}")?;
    Ok(())
}
