use hide_console::hide_console;
use rayon::prelude::*;
use regex::Regex;
use sonic_rs::{JsonContainerTrait, JsonValueTrait, Value};
use std::{env, fs, process::exit};

mod globals;
use globals::*;
mod simple_utils;
use simple_utils::*;

fn get_browser_download_url(repo: &str, sources: &str, simd: Option<&str>) -> Option<String> {
    let api_url = format!("{}{}{}", API, repo, sources);
    let beta = api_url.ends_with(BETA_SOURCES);
    let sources = "sources.json";

    let status = simple_spawn("curl", &["-#L", &api_url, "-o", &sources], false);
    if status != 0 {
        exit(status);
    }

    let json = fs::read_to_string(&sources).unwrap();
    fs::remove_file(&sources).unwrap();
    let root = sonic_rs::from_str::<Value>(&json).unwrap();
    let value = if beta {
        &root[0]["assets"]
    } else {
        &root["assets"]
    };
    let mut assets = value
        .as_array()
        .unwrap()
        .par_iter()
        .filter_map(|asset| {
            let url = asset["browser_download_url"].as_str()?;
            if url.to_lowercase().ends_with(TARGET_QUERY) {
                Some(url.to_owned())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let mut simd_list = assets
        .par_iter()
        .map(|url| {
            let _t = url.replace(&format!("_{}", TARGET_QUERY), "");
            let _i = _t.rfind("_").unwrap();
            _t[_i + 1..].to_owned()
        })
        .collect::<Vec<_>>();
    simd_list.sort_by(|a, b| natord::compare(a, b));

    let max_simd = if let Some(simd) = simd {
        simd
    } else if is_x86_feature_detected!("avx512f") {
        "avx512"
    } else if is_x86_feature_detected!("avx2") {
        "avx2"
    } else if is_x86_feature_detected!("avx") {
        "avx"
    } else if is_x86_feature_detected!("sse4.2") {
        "sse4"
    } else if is_x86_feature_detected!("sse3") {
        "sse3"
    } else if is_x86_feature_detected!("sse2") {
        "sse2"
    } else {
        ""
    };

    println!(
        "Currently supported CPU instructions sets: {}. {}: {}",
        simd_list.join(", "),
        if simd.is_none() {
            "Detected CPU maximum capability"
        } else {
            "You selected"
        },
        max_simd.to_uppercase()
    );

    assets.retain(|url| {
        if max_simd.is_empty() {
            false
        } else {
            url.to_lowercase()
                .ends_with(&format!("{}_{}", max_simd, TARGET_QUERY).to_lowercase())
        }
    });

    if assets.is_empty() {
        eprintln!("ERROR: No match or your CPU is not supported.");
        return None;
    }
    Some(assets.into_iter().next().unwrap())
}

fn help() {
    let readme = include_str!("../readme.md")
        .replacen("\n", &format!(" v{}\n", env!("CARGO_PKG_VERSION")), 1)
        .replace("(../../", &format!("({}/", env!("CARGO_PKG_REPOSITORY")));
    termimad::print_text(&readme);
    exit(0)
}

fn main() {
    env::set_current_dir(env::var("TMP").unwrap_or_else(
        // Use current directory if TMP is not set
        |_| ".".to_owned(),
    ))
    .unwrap();

    let mut repo = REPO.to_owned();
    let mut sources = RELEASE_SOURCES.to_owned();

    let mut force = false;
    let mut clearuserdata = false;
    let mut simd: Option<String> = None;

    let mut installer_args: Vec<String> = vec![];
    let mut warn_args: Vec<String> = vec![];

    let re_kv = Regex::new(r"^/(?P<key>\w+)=(?P<val>.+)$").unwrap();
    let re_flag = Regex::new(r"^/(?P<flag>.+)$").unwrap();

    let mut args: Vec<String> = env::args().skip(1).collect();

    for i in (0..args.len()).rev() {
        if let Some(capture) = re_flag.captures(&args[i]) {
            let flag = &capture["flag"];

            let matched = match flag {
                "?" => {
                    help();
                    true
                }
                "help" => {
                    help();
                    true
                }
                "silent" => {
                    hide_console();
                    true
                }
                "force" => {
                    force = true;
                    true
                }
                "clearuserdata" => {
                    clearuserdata = true;
                    true
                }
                _ => false,
            };
            if matched {
                args.remove(i);
            }
        }
        if let Some(capture) = re_kv.captures(&args[i]) {
            let key = capture["key"].to_lowercase();
            let val = capture["val"].to_owned();

            // Begin parsing key/value pair
            let matched = match key.as_str() {
                "repo" => {
                    repo = format!("/{}", val);
                    true
                }
                "simd" => {
                    simd = Some(val);
                    true
                }
                "channel" => {
                    if val == "beta" {
                        eprintln!("WARNING: Using 'beta' channel...");
                        sources = BETA_SOURCES.to_owned();
                    } else if val != "latest" {
                        eprintln!(
                            "WARNING: Channel '{}' is unknown. Currently only knows 'beta', falling back to release...",
                            val
                        )
                    }
                    true
                }
                _ => false,
            };
            if matched {
                args.remove(i);
            }
        } else {
            // Won't stay long (dynamic), it's okay
            warn_args.push(args[i].clone());
        }
    }

    if !warn_args.is_empty() {
        let formated_warn_args = warn_args
            .par_iter()
            .map(|a| format!("'{}'", a))
            .collect::<Vec<_>>()
            .join(", ");
        eprintln!(
            "WARNING: {} unknown arguments passed. Passing thru installer arguments...",
            formated_warn_args
        );
    }

    let browser_download_url = get_browser_download_url(&repo, &sources, simd.as_deref());
    if let Some(url) = browser_download_url {
        let download_path = "setup.exe";
        let status = aria2_downloader(&url, &download_path, None);
        if status != 0 {
            exit(status)
        }
        simple_open(&download_path, true);
    } else {
        exit(2)
    }
}
