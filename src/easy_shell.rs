pub mod _easy_shell_ {
    use std::env;
    use std::path::Path;
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
                Err(e) => { eprintln!("{}", e);Err("Failed to Change dir") }
            }
        }
    }
}