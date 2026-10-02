use const_format::concatcp;

pub const API: &str = "https://api.github.com/repos";
pub const REPO: &str = "/gz83/thorium";
pub const BETA_SOURCES: &str = "/releases";
pub const RELEASE_SOURCES: &str = concatcp!(BETA_SOURCES, "/latest");
pub const TARGET_QUERY: &str = "mini_installer.exe";
