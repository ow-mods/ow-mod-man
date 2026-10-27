use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

use anyhow::{Context, Result, bail};

const NATIVE_LIB_PREFIXES: &[&str] = &["libmono", "libMono", "libgdiplus", "libssl", "libcrypto"];

/// Prepares a Mono runtime tree for the GUI AppImage.
///
/// Expects the Ubuntu Mono packages (`mono-devel` and `libgdiplus`) to already
/// be installed on the build machine.
pub fn bundle_appimage_mono() -> Result<()> {
    if std::env::consts::OS != "linux" {
        bail!("AppImage Mono bundling is only supported on Linux");
    }

    let multiarch = match std::env::consts::ARCH {
        "x86_64" => "x86_64-linux-gnu",
        "aarch64" => "aarch64-linux-gnu",
        arch => bail!("Unsupported architecture for AppImage Mono bundling: {arch}"),
    };

    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf();
    let dest = repo_root.join("owmods_gui/backend/appimage-mono");

    // Keep the checked-in .gitignore, replace only generated content.
    for dir in ["usr", "etc"] {
        let path = dest.join(dir);
        if path.exists() {
            fs::remove_dir_all(&path)
                .with_context(|| format!("Removing old {}", path.display()))?;
        }
    }
    let wrapper = dest.join("mono");
    if wrapper.exists() {
        fs::remove_file(&wrapper)?;
    }

    fs::create_dir_all(dest.join("usr/bin"))?;
    fs::create_dir_all(dest.join("usr/lib"))?;
    fs::create_dir_all(dest.join("etc"))?;

    copy_matching(Path::new("/usr/bin"), &dest.join("usr/bin"), |name| {
        name == "mono" || name == "mono-sgen"
    })?;
    copy_dir(Path::new("/usr/lib/mono"), &dest.join("usr/lib/mono"))?;
    copy_dir(Path::new("/etc/mono"), &dest.join("etc/mono"))?;

    let multiarch_lib = format!("/usr/lib/{multiarch}");
    for src in [Path::new("/usr/lib"), Path::new(&multiarch_lib)] {
        if src.is_dir() {
            copy_matching(src, &dest.join("usr/lib"), |name| {
                NATIVE_LIB_PREFIXES
                    .iter()
                    .any(|prefix| name.starts_with(prefix))
            })?;
        }
    }

    fs::write(&wrapper, include_str!("mono_templates/mono"))?;
    #[cfg(unix)]
    {
        let mut permissions = fs::metadata(&wrapper)?.permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(&wrapper, permissions)?;
    }

    let copyright = Path::new("/usr/share/doc/mono-runtime/copyright");
    if copyright.is_file() {
        fs::copy(copyright, dest.join("COPYING"))?;
    }

    println!("Bundled Mono into {}", dest.display());
    Ok(())
}

fn copy_matching(src_dir: &Path, dest_dir: &Path, matches: impl Fn(&str) -> bool) -> Result<()> {
    for entry in
        fs::read_dir(src_dir).with_context(|| format!("Reading directory {}", src_dir.display()))?
    {
        let entry = entry?;
        let name = entry.file_name();
        let Some(name) = name.to_str() else {
            continue;
        };
        if matches(name) {
            cp_a(&entry.path(), &dest_dir.join(name))?;
        }
    }
    Ok(())
}

fn copy_dir(src: &Path, dest: &Path) -> Result<()> {
    if !src.is_dir() {
        bail!("Expected {} to exist", src.display());
    }
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent)?;
    }
    cp_a(src, dest)
}

fn cp_a(src: &Path, dest: &Path) -> Result<()> {
    let status = Command::new("cp")
        .arg("-a")
        .arg("--")
        .arg(src)
        .arg(dest)
        .status()
        .with_context(|| format!("Running cp for {}", src.display()))?;
    if !status.success() {
        bail!("Failed to copy {}", src.display());
    }
    Ok(())
}
