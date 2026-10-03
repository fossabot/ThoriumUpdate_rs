use hide_console::hide_console;
use rayon::prelude::*;
use regex::Regex;
use std::{env, process::exit};
use termimad::MadSkin;
use termimad::crossterm::style::Attribute::*;
use versions::Versioning;

mod simple_utils;
use simple_utils::*;
mod installer;
use installer::*;

fn help() {
    let mut readme = include_str!("../readme.md")
        .replace("(../../", &format!("({}/", env!("CARGO_PKG_REPOSITORY")));
    readme = regex_replace_all(&readme, r"(?m)^.*```.*$", "");
    readme = regex_replace_all(&readme, r"> \[!(\w+)\]\n> ", "> **$1:** ");
    readme = regex_replace_all(&readme, r"\[(.+)\]\((.+)\)(.)", "$1 [$2]$3");
    readme = regex_replace_all(&readme, r"(?m)^[^\S\n]+>", ">");
    readme = readme.replacen("\n", &format!(" v{}\n", env!("CARGO_PKG_VERSION")), 1);
    let mut skin = MadSkin::default();
    skin.italic.add_attr(Underlined);
    println!();
    skin.print_text(&readme);
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

    // let mut installer_args: Vec<String> = vec![];
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
                continue;
            }
        }
        if let Some(capture) = re_kv.captures(&args[i]) {
            let key = capture["key"].to_lowercase();
            let val = capture["val"].to_owned();
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
                continue;
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

    let installer_info = get_installer_info(&repo, &sources, simd.as_deref());
    if let Some(info) = installer_info {
        let v1 = Versioning::new(&info.version).unwrap_or_default();
        println!("Currently installed: {}", v1.to_string());
        let v2 = Versioning::new(uninstall_reg_get_string("Version").unwrap_or_default())
            .unwrap_or_default();
        println!("Upstream: {}", v2.to_string());
        if v1 > v2 || force {
            let download_path = "setup.exe";
            let status = aria2_downloader(&info.url, &download_path, None);
            if status != 0 {
                exit(status)
            }
            simple_open(&download_path, false);
            println!("Thorium browser has been updated.");
        } else {
            println!("You are up to date.");
        }
    } else {
        exit(2)
    }
}
