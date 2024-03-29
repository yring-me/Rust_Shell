use std::env;
use std::io::{stdout, Write};
use crate::config::_config_::{Command, Shell};
use lazy_static::lazy_static;
use crate::file_system::_easy_shell_::{Cd, Clear, Mkdir, Pwd, Rm, Touch};
use crate::ls::_ls_::Ls;

#[macro_export]
macro_rules! color {
    ("red") => {"\x1b[31m"};
    ("green") => {"\x1b[32m"};
    ("yellow") => {"\x1b[33m"};
    ("blue") => {"\x1b[34m"};
    ("magenta") => {"\x1b[35m"};
    ("cyan") => {"\x1b[36m"};
    ("reset") => {"\x1b[0m"}
}

pub trait SyscallHandler {
    fn handler(_cmd: &Command) -> Result<&str,String> {
        println!("command no found!");
        Ok("")
    }
}

lazy_static!{pub static ref CMD_LIST: Vec<Shell> = vec![Shell{name:"cd",handler:Cd::handler},
                                                        Shell{name:"pwd",handler:Pwd::handler},
                                                        Shell{name:"mkdir",handler:Mkdir::handler},
                                                        Shell{name:"ls",handler:Ls::handler},
                                                        Shell{name:"rm",handler:Rm::handler},
                                                        Shell{name:"clear",handler:Clear::handler},
                                                        Shell{name:"touch",handler:Touch::handler}];}


pub mod _config_{
    use crate::config::{CMD_LIST};

    #[derive(Debug)]
    pub struct Command {
        pub(crate) command:String,
        pub(crate) args:Vec<String>,
    }
    #[derive(Debug)]
    pub struct Shell{
        pub(crate) name:&'static str,
        pub(crate) handler:fn(&Command)->Result<&str,String>,
    }

    impl Command {
        pub fn new() -> Command{
            Command{
                command: String::from(""),
                args:vec![]
            }
        }
        pub fn set_config(ipt:String, cfg: &mut Command) {
            let mut ipt = ipt.trim().split_whitespace();
            cfg.command = ipt.next().unwrap().parse().unwrap();
            for arg in ipt{
                cfg.args.push(arg.parse().unwrap());
            }
        }
        pub fn reset_config(cfg:&mut Command){
            cfg.command = String::from("");
            cfg.args.clear();
        }
        pub fn execute(input_command: &mut Command) -> bool{
            for index in 0..CMD_LIST.len(){
                let cmd = CMD_LIST.get(index).unwrap();
                if cmd.name == input_command.command.as_str(){
                    return match { (cmd.handler)(&input_command) } {
                        Ok(_) => {
                            Command::reset_config(input_command);
                            true
                        }
                        Err(e) => {
                            println!("{}", e);
                            Command::reset_config(input_command);
                            true
                        }
                    };
                }
            }
            return false;
        }
    }
}

impl SyscallHandler for Command{
    fn handler(_cmd:&Command) -> Result<&str,String>{
        let current_dir = env::current_dir().unwrap().into_os_string().into_string().unwrap();
        println!(" {}  {}",color!("blue"),current_dir);
        print!("{}❯ {}",color!("green"),color!("reset"));
        stdout().flush().unwrap();
        Ok("")
    }
}
