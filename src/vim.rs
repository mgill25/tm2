use std::io::{BufRead, BufReader};
use std::process::*;

pub fn switch_colorscheme() {
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
                ml.to_string()
            })
            .filter(|ml| {
                !ml.starts_with("Plug")
                    && !ml.starts_with('"')
                    && !ml.starts_with("call")
                    && !ml.starts_with('\\')
            })
            .last();

        if let Some(theme_line) = theme_match {
            let vim_theme = theme_line.split(' ').collect::<Vec<&str>>()[1];
            println!("{}", &vim_theme);
        }
    }
}
