use crate::instructions::Command;
use crate::instructions::CommandOption;
use crate::instructions::Instruction;
use crate::s;
use crate::FLAGS;
use log::{debug, info};

/// Global handler function for all Instructions
/// Invokes unique handlers for each specific command/option we get from the Instruction
pub fn handle(ins: Instruction) {
    debug!("HANDLE INS = {:?}", ins);
    match ins {
        Instruction { command, option } => {
            debug!("\tCMD = {:?}", command);
            debug!("\tOPT = {:?}", option);
            match command {
                Some(Command::Help(bin_name)) => cmd_print_help(bin_name),
                Some(Command::ListThemes) => cmd_list_themes(),
                Some(Command::SwitchTheme(new_theme)) => match option {
                    Some(CommandOption::SwitchWithVim) => cmd_handle_switch_with_vim(&new_theme),
                    _ => cmd_handle_switch(&new_theme),
                },
                _ => {}
            }
        }
    }
}

pub fn cmd_print_help(bin_name: String) {
    let topline = format!("{} [options] <parameter>\n", bin_name);
    let mut secondline = s("\nOPTIONS:\n");
    for flag in FLAGS.to_vec() {
        secondline.push_str(format!("\t{}: \t {}\n", flag.name, flag.desc).as_str());
    }
    let usage = format!("{}{}", topline, secondline);
    println!("Usage:\n\t{}", usage);
}

fn cmd_list_themes() {
    info!("This should list all the available themes!");
}

fn cmd_handle_switch(new_theme: &String) {
    info!("This should switch the terminal theme to {}", new_theme);
}

fn cmd_handle_switch_with_vim(new_theme: &String) {
    cmd_handle_switch(&new_theme);
    info!("This will also try to change the same theme in neovim");
}
