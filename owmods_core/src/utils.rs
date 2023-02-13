use std::path::{Path, PathBuf};

pub fn fix_version(version: &str) -> String {
    let mut str = version.to_owned();
    while str.starts_with('v') {
        str = str.strip_prefix('v').unwrap_or(&str).to_string();
    }
    str
}

pub fn get_end_of_url(url: &str) -> &str {
    url.split('/').last().unwrap_or(url)
}

pub fn check_file_matches_paths(path: &Path, to_check: &[PathBuf]) -> bool {
    for check in to_check.iter() {
        if check.file_name().unwrap_or(check.as_os_str())
            == path.file_name().unwrap_or(path.as_os_str())
            || path.starts_with(check)
        {
            return true;
        }
    }
    false
}
