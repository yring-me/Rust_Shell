pub mod _easy_shell_ {
    use std::env;
    use std::fs::{create_dir, create_dir_all};
    use std::path::Path;
    use crate::color;
    use crate::config::_config_::Command;
    use crate::config::SyscallHandler;

    pub struct Pwd {}
    impl SyscallHandler for Pwd{
        fn handler(_cmd:&Command) -> Result<&str,&str>{
            let current_dir = env::current_dir().unwrap().into_os_string().into_string().unwrap();
            println!("{}",current_dir);
            Ok("")
        }
    }
    
    pub struct Cd{}

    impl SyscallHandler for Cd{
        fn handler(cmd: &Command) -> Result<&str, &str> {
            let path = &cmd.args.iter().peekable().peek().map_or("/", |x| *x);
            let curr_dir = Path::new(path);
            let ret = env::set_current_dir(curr_dir);
            match ret {
                Ok(_r) => {Ok("")},
                Err(e) => { println!("{}", e);
                                  Err("Failed to Change dir") }
            }
        }
    }

    pub struct Mkdir<'mkdir>{
        pub(crate) is_p:bool,
        pub(crate) file_name:Vec<&'mkdir str>
    }
    
    impl SyscallHandler for Mkdir<'static>{
        fn handler(cmd: &Command) -> Result<&str, &str> {
            let mut mkdir = Mkdir{is_p:false,file_name:vec![]};
            for arg in &cmd.args{
                if arg.starts_with("-"){
                    match arg.as_str() {
                        "-p" => {mkdir.is_p = true;continue}
                        _ => {println!("{}Warning invalid arguments{}{}",color!("yellow"),&arg,color!("reset"));continue;}
                    }
                }
                mkdir.file_name.push(&arg);
            }

            for file_name in mkdir.file_name{
                if mkdir.is_p == true{
                    let ret = create_dir_all(Path::new(file_name));
                    match ret {
                        Ok(_)=> {}
                        Err(_) => {println!("Failed to create {}",file_name)}
                    }
                }

                else {
                    let ret = create_dir(Path::new(file_name));
                    match ret {
                        Ok(_)=> {}
                        Err(e) => {
                            // ./123
                            let len = file_name.match_indices("/").collect::<Vec<_>>().len();
                            if len == 1 && file_name.starts_with("./"){
                                println!("{}",e);
                                println!("Failed to create {}",file_name);}

                            // ./123/123
                            else if len > 1 {
                                println!("{}",e);
                                println!("Try to use -p to create a cursive dir");
                            }
                        }
                    }
                }
            }
            Ok("")
        }
    }
}