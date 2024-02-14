#[macro_export]
macro_rules! shell_id {
    ("cd") => {1}
}


pub mod _config_{
    #[derive(Debug)]
    pub struct Command {
        command:String,
        args:Vec<String>,
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
    }
}
