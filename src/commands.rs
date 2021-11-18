use crate::instructions::Command;
use crate::instructions::CommandOption;
use crate::instructions::Instruction;
use crate::s;
use crate::FLAGS;
use log::{debug, info};

const DEFAULT_THEME_ROOT: &str = "/Users/gill/.config/alacritty/alacritty-theme/themes";

/// Global handler function for all Instructions
/// Invokes unique handlers for each specific command/option we get from the Instruction
pub fn handle(ins: Instruction) {
    debug!("HANDLE INS = {:?}", ins);
    let handler = Handler {};
    match ins {
        Instruction { command, option } => {
            debug!("\tCMD = {:?}", command);
            debug!("\tOPT = {:?}", option);
            match command {
                Some(Command::Help(bin_name)) => handler.print_help(&bin_name),
                Some(Command::ListThemes) => handler.list_themes(),
                Some(Command::SwitchTheme(new_theme)) => match option {
                    Some(CommandOption::SwitchWithVim) => handler.switch_with_vim(&new_theme),
                    _ => handler.switch(&new_theme),
                },
                _ => {}
            }
        }
    }
}

struct Handler {}

impl Handler {
    pub fn print_help(&self, bin_name: &String) {
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
        use std::fs;
        // info!("This should list all the available themes!");
        // How do I list files in a directory using Rust?
        // exa -lah ~/.config/alacritty/alacritty-theme/themes
        let themes_filepath = DEFAULT_THEME_ROOT;
        let themes_dir = fs::read_dir(themes_filepath).unwrap();

        for theme in themes_dir {
            let theme_fname = theme.unwrap().file_name();
            let theme_fstr = theme_fname.to_str().unwrap();

            // split the string and remove the extension
            let theme_name = theme_fstr.split(".").collect::<Vec<&str>>();
            println!("{}", &theme_name[0]);
        }
    }

    fn switch(&self, new_theme: &String) {
        info!("This should switch the terminal theme to {}", new_theme);
    }

    fn switch_with_vim(&self, new_theme: &String) {
        self.switch(&new_theme);
        info!("This will also try to change the same theme in neovim");
    }
}
