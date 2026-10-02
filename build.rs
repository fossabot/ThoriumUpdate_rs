use chrono::{Datelike, Local};
use std::fs;

#[cfg(windows)]
fn main() -> std::io::Result<()> {
    // Fix this if you want to use a custom icon or manifest
    let app_icon = "app.ico";
    let manifest = "app.manifest";

    let mut res = winresource::WindowsResource::new();
    let icon_is_file = fs::metadata(app_icon)
        .map(|m| m.is_file())
        .unwrap_or_else(|_| false);
    if icon_is_file {
        res.set_icon(app_icon);
    }
    let manifest_is_file = fs::metadata(manifest)
        .map(|m| m.is_file())
        .unwrap_or_else(|_| false);
    if manifest_is_file {
        res.set_manifest_file(manifest);
    }

    let year = Local::now().year();
    let origin = "thrup.exe";
    res.set("CompanyName", env!("CARGO_PKG_AUTHORS"))
        .set("FileDescription", env!("CARGO_PKG_DESCRIPTION"))
        // .set("FileVersion", "10.0.26100.1 (WinBuild.160101.0800)")
        .set("InternalName", origin)
        .set(
            "LegalCopyright",
            &format!("© {} {}. All rights reserved.", year, env!("CARGO_PKG_AUTHORS")),
        )
        .set("OriginalFilename", origin)
        .set("ProductName", env!("CARGO_PKG_DESCRIPTION"))
        // .set("ProductVersion", "10.0.26100.1")
        ;
    res.compile()?;
    Ok(())
}
