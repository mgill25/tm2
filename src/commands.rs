use crate::fileutils;
use crate::instructions::Command;
use crate::instructions::CommandOption;
use crate::instructions::Instruction;
use crate::s;
use crate::FLAGS;
use log::{debug, info};
use std::fs;

const CURR_THEME_STATE_FILE: &str = "/Users/gill/.config/alacritty/.current_theme";
const DEFAULT_THEME_ROOT:    &str = "/Users/gill/.config/alacritty/alacritty-theme/themes";

const ALACRITTY_CONFIG_BASE:  &str = "/Users/gill/.config/alacritty/base.yml";
const ALACRITTY_CONFIG_FINAL: &str = "/Users/gill/.config/alacritty/alacritty.yml";

/// Global handler function for all Instructions
/// Invokes unique handlers for each specific command/option we get from the Instruction
pub fn handle(ins: Instruction) {
    debug!("HANDLE INS = {:?}", ins);
    let handler = Handler {};
    let Instruction { command, option } = ins;
    debug!("\tCMD = {:?}", command);
    debug!("\tOPT = {:?}", option);
    match command {
        Some(Command::Help(bin_name)) => handler.print_help(&bin_name),
        Some(Command::ListThemes) => handler.list_themes(),
        Some(Command::SearchTheme(theme_name)) => handler.search_theme(&theme_name),
        Some(Command::CurrentTheme) => handler.show_current_theme(),
        Some(Command::SwitchTheme(new_theme)) => match option {
            Some(CommandOption::SwitchWithVim) => handler.switch_with_vim(&new_theme),
            _ => handler.switch(&new_theme),
        },
        _ => {}
    }
}

struct Handler {}

impl Handler {
    pub fn print_help(&self, bin_name: &str) {
        let topline = format!("{} [options] <parameter>\n", bin_name);
        let mut secondline = s("\nOPTIONS:\n");
        for flag in FLAGS.to_vec() {
            secondline.push_str(format!("\t{}: \t {}\n", flag.name, flag.desc).as_str());
        }
        let usage = format!("{}{}", topline, secondline);
        println!("Usage:\n\t{}", usage);
    }

    /// List the themes in the root theme directory
    fn list_themes(&self) {
        // info!("This should list all the available themes!");
        // How do I list files in a directory using Rust?
        // exa -lah ~/.config/alacritty/alacritty-theme/themes
        let themes_filepath = DEFAULT_THEME_ROOT;
        let themes_dir = fs::read_dir(themes_filepath).unwrap();

        for theme in themes_dir {
            let theme_fname = theme.unwrap().file_name();
            let theme_fstr = theme_fname.to_str().unwrap();
            // split the string and remove the extension
            let theme_name = theme_fstr.split('.').collect::<Vec<&str>>();
            println!("{}", &theme_name[0]);
        }
    }

    /// Search for a given theme by name (and later by regex/glob pattern)
    /// Prints the name of the theme if a match is found.
    /// Prints "not found" otherwise
    fn search_theme(&self, theme_name: &str) {
        let themes_filepath = DEFAULT_THEME_ROOT;
        let themes_dir = fs::read_dir(themes_filepath).unwrap();
        let mut found = false;
        for curr_theme in themes_dir {
            let curr_theme_fname = curr_theme.unwrap().file_name();
            let curr_theme_fstr = curr_theme_fname.to_str().unwrap();
            let curr_theme_name = curr_theme_fstr.split('.').collect::<Vec<&str>>()[0];
            if s(curr_theme_name).starts_with(theme_name) {
                println!("{}", curr_theme_name);
                found = true;
            }
        }
        if !found {
            println!("Not found");
        }
    }

    fn show_current_theme(&self) {
        let state_file_content = fileutils::read_from_file(CURR_THEME_STATE_FILE);
        println!("{}", &state_file_content);
    }

    /**
     * (shell equivalent)
     * NEW_THEME=$1
     * rm "$HOME/.config/alacritty/alacritty.yml" && cat "$HOME/.config/alacritty/base.yml" "$HOME/.config/alacritty/alacritty-theme/themes/$NEW_THEME.yml" > ~/.config/alacritty/alacritty.yml
     * # also write the current theme name to a file
     * touch "$HOME/.config/alacritty/.current_theme"
     * rm "$HOME/.config/alacritty/.current_theme" && echo "$NEW_THEME" > "$HOME/.config/alacritty/.current_theme"
     */
    fn switch(&self, new_theme: &str) {
        info!("This should switch the terminal theme to {}", new_theme);
        let new_theme_config_file = format!("/Users/gill/.config/alacritty/alacritty-theme/themes/{}.yml", new_theme);
        if !fileutils::exists(new_theme_config_file.as_str()) {
            println!("ERROR: Theme not found!");
            return
        }
        fileutils::remove(ALACRITTY_CONFIG_FINAL);
        let base_config_content = fileutils::read_from_file(ALACRITTY_CONFIG_BASE);
        let new_theme_content = fileutils::read_from_file(new_theme_config_file.as_str());
        let final_content = format!("{}{}", base_config_content, new_theme_content);
        fileutils::write(ALACRITTY_CONFIG_FINAL, final_content);
        fileutils::remove(CURR_THEME_STATE_FILE);
        fileutils::write(CURR_THEME_STATE_FILE, s(new_theme));
    }

    fn switch_with_vim(&self, new_theme: &str) {
        self.switch(new_theme);
        info!("This will also try to change the same theme in neovim");
    }
}