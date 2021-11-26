use std::io::{BufRead, BufReader};
use std::process::*;
use log::debug;
use crate::fileutils;

pub fn switch_colorscheme(new_theme: &str) {
    debug!("Trying to switch the vim colorscheme");
    let mut child = Command::new("/usr/local/bin/ag")
        .args(["colorscheme", "/Users/gill/.vimrc"])
        .stdout(Stdio::piped())
        .spawn()
        .expect("Unable to spawn program");

    if let Some(stdout) = &mut child.stdout {
        let lines = BufReader::new(stdout).lines();
        let theme_match = lines
            .into_iter()
            .map(|line| {
                let line_orig = line.unwrap();
                let line_split = line_orig.trim_start().split(':').collect::<Vec<&str>>();
                let ml = line_split[1].trim_start();
                debug!("ml = {:?}", &ml);
                ml.to_string()
            })
            .filter(|ml| {
                !ml.starts_with("Plug")
                    && !ml.starts_with('"')
                    && !ml.starts_with("call")
                    && !ml.starts_with('\\')
            })
            .last();

        if let Some(ref theme_line) = theme_match {
            let new_theme_vim_cmd = format!("colorscheme {}", &new_theme);
            let ok = fileutils::sed("/Users/gill/.vimrc",
                    theme_line,
                    new_theme_vim_cmd.as_str()
            ).is_ok();
            if !ok {
                println!("ERROR: Something went wrong during vim substitution. Check your file!");
            }
        }
    }
}
