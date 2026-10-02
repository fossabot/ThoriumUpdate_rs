use std::{env, fs, process::Command};

// pub fn simple_which(keyword: &str) -> bool {
//     // Current directory first
//     if let Ok(path) = which_in(keyword, std::env::current_dir()) {
//         return path.is_file();
//     }
//     // Then PATH
//     if let Ok(path) = which(keyword) {
//         return path.is_file();
//     }
//     false
// }
//

const ARIA2_CMD: &str = ".\\aria2c.exe";
#[cfg(all(target_os = "windows", target_pointer_width = "32"))]
const ARIA2_BIN: &[u8] = include_bytes!("../assets/aria2c_x86.exe");
#[cfg(all(target_os = "windows", target_pointer_width = "64"))]
const ARIA2_BIN: &[u8] = include_bytes!("../assets/aria2c_x64.exe");
const ARIA2_CONFIG: &str = include_str!("../assets/aria2.conf");
const ARIA2_CONF: &str = ".\\aria2.conf";

pub fn aria2_downloader(url: &str, output_path: &str, config: Option<&str>) -> i32 {
    if let Some(config) = config {
        fs::write(ARIA2_CONF, config).unwrap();
    } else {
        fs::write(ARIA2_CONF, ARIA2_CONFIG).unwrap();
    }
    fs::write(ARIA2_CMD, ARIA2_BIN).unwrap();
    let status = simple_spawn(
        ARIA2_CMD,
        &[
            url,
            &format!("--conf-path={}", ARIA2_CONF),
            "-o",
            output_path,
        ],
        false,
    );
    fs::remove_file(ARIA2_CMD).unwrap();
    fs::remove_file(ARIA2_CONF).unwrap();
    status
}

pub fn simple_spawn(cmd: &str, args: &[&str], suppress_error: bool) -> i32 {
    let mut process = Command::new(cmd)
        .args(args)
        .spawn()
        .map_err(|e| format!("{} failed: {}", cmd, e))
        .unwrap();

    let status = process.wait().map_err(|e| e.to_string()).unwrap();
    if !status.success() {
        if !suppress_error {
            eprintln!("ERROR: {} ({})", cmd, status);
        }
        if let Some(code) = status.code() {
            return code;
        } else {
            return 1;
        }
    }
    0
}

pub fn simple_open(path: &str, as_location: bool) {
    let opener = "explorer.exe";
    if as_location {
        simple_spawn(opener, &["/select,", path], true);
    } else {
        simple_spawn(opener, &[path], true);
    }
}
