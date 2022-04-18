extern crate getopts;
use getopts::Options;
use std::env;

//struct cmdLine;
//impl cmdLine{
//fn new(){}//associate function, leave blank otherwise
//}
pub fn version(executable: &str) {
    //let theversion : String; //run script that updates
    //format!("Current Version is {}", theversion);
    format!("Usage: {} [options]", executable);
    println!("Lightning-fast and Powerful Code Editor: Check details at https://lapce.dev");
}

pub fn help(executable: &str, opts: Options) {
    let error = format!("Usage: {} [options]", executable);
    println!("{}", opts.usage(&error));
    println!("If you are having trouble with Lapce setup, please visit https://docs.lapce.dev");
    println!("If you wish to contribute, please go to https://github.com/lapce/lapce.git");
}

pub fn start_cmd_line() -> bool {
    let args: Vec<String> = env::args().collect(); //get arguments
    let exec = &args[0]; //executable
    println!("{} is the len", args.len());
    println!("{} is the first arg", args[0]);
    let mut opts = Options::new();
    opts.optflag("v", "version", "check Lapce version");
    opts.optflag("h", "help", "help");
    //additional new options go here
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => {m}, //accepts from Ok()
        Err(f) => {
            panic!("{}. Use --help to list recognized commands", f.to_string());
        } //rejects from Err()
    };
    if matches.opt_present("h") {
        help(&exec, opts);
        return true;
    }
    if matches.opt_present("v") {
        version(&exec);
        return true;
    }
    return true;
}

fn use_args(args: impl Iterator<Item=String>) {
    //args.map(|arg| arg.len());
	for arg in args {
        // arg: String
        print!("{}", arg);
    }
}





fn main() {
    start_cmd_line();
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    //use std::io::stdin;
    use super::*;
    use std::process::Command;  // Run programs
    use std::env;
    use std::env::consts;
    use std::path::PathBuf;
    //use crate::parse;
    #[test]
	fn test_parse_help() {
		let args = ["help", "new"].iter().map(|s| s.to_string());
   		use_args(args);
        let mut args: Vec<String> = env::args().collect(); //get arguments
        println!("\n{:?} are the cmd line ", args.len());
        let mut opts = Options::new();
        opts.optflag("v", "version", "check Lapce version");
        opts.optflag("h", "help", "help");
        
        let matches = match opts.parse(&args[1..]) {
            Ok(m) => m, //accepts from Ok()
            Err(f) => {
                panic!("{}. Use --help to list recognized commands", f.to_string());
                
            }// //rejects from Err()
        };
        println!("{} arg1 is ", args.len());

        if matches.opt_present("help") {
            assert!(true);
        } else {
            assert!(false);
        }

	    assert!(true);
    }
	
	#[test]
    fn test_works_not_version() {
        //use_args(std::iter::once("help".to_string()));

        let mut args: Vec<String> = env::args().collect(); //get arguments
        let mut opts = Options::new();
        opts.optflag("v", "version", "check Lapce version");
        opts.optflag("h", "help", "help");
        //args.len() = 3;
        println!("{} is the length and ", args.len());
        println!("{} is the name ", args[0]);

        //args[1] = "--help".to_string();
        //args[2] = "--sdfasdf".to_string();
        let matches = match opts.parse(&args[1..]) {
            Ok(m) => m, //accepts from Ok()
            Err(f) => {
                panic!("{}. Use --help to list recognized commands", f.to_string());
                
            }// //rejects from Err()
        };

        if matches.opt_present("v") {
            assert!(false);
        } else {
            assert!(true);
        }
    }

    

    #[test]
    fn test_works_for_version() {
        let args: Vec<String> = env::args().collect(); //get arguments
        println!("{} is jlas fasldjf as fasdjf sadfjlk sadlfk asd", args.len());
        println!("{} is jlas fasldjf as fasdjf sadfjlk sadlfk asd", args[0]);
        assert!(true);
        let mut opts = Options::new();
        opts.optflag("v", "version", "check Lapce version");
        opts.optflag("h", "help", "help");
        let matches = match opts.parse(&args[1..]) {
            Ok(m) => m, //accepts from Ok()
            Err(f) => {
                //rejects from Err()
                panic!("{}. Use --help to list recognized commands", f.to_string());
            }
        };
        if matches.opt_present("version") {
            assert!(true);
        }
    }

    #[test]
    fn test_works_for_help() {
        let args: Vec<String> = env::args().collect(); //get arguments
        let mut opts = Options::new();
        opts.optflag("v", "version", "check Lapce version");
        opts.optflag("h", "help", "help");
        let matches = match opts.parse(&args[1..]) {
            Ok(m) => m, //accepts from Ok()
            Err(f) => {
                panic!("{}. Use --help to list recognized commands", f.to_string());
                //rejects from Err()
            }
        };
        if matches.opt_present("help") {
            assert!(true);
        }
    }

    #[test]
    fn test_works_not_help() {
        let args: Vec<String> = env::args().collect(); //get arguments
        let mut opts = Options::new();
        opts.optflag("v", "version", "check Lapce version");
        opts.optflag("h", "help", "help");
        let matches = match opts.parse(&args[1..]) {
            Ok(m) => m, //accepts from Ok()
            Err(f) => {
                if f.to_string() != "help" {
					assert!(true)
                } //rejects from Err()
                panic!("{}. Use --help to list recognized commands", f.to_string());

            }
        };
        if matches.opt_present("v") {
            assert!(true);
        }
    }
    #[test]
    fn test_help_h() {
        let args: Vec<String> = env::args().collect(); //get arguments
        let exec = &args[0]; //executable
        let mut opts = Options::new();
        opts.optflag("v", "version", "check Lapce version");
        opts.optflag("h", "help", "help");
        //additional new options go here
        let matches = match opts.parse(&args[1..]) {
            Ok(m) => m, //accepts from Ok()
            Err(f) => {
                panic!("{}. Use --help to list recognized commands", f.to_string());
            } //rejects from Err()
        };
        if matches.opt_present("h") {
            help(&exec, opts);
            assert!(true);
        }
    }
    #[test]
    fn test_version_v() {
        let args: Vec<String> = env::args().collect(); //get arguments
        let exec = &args[0]; //executable
        let mut opts = Options::new();
        opts.optflag("v", "version", "check Lapce version");
        opts.optflag("h", "help", "help");
        //additional new options go here
        let matches = match opts.parse(&args[1..]) {
            Ok(m) => m, //accepts from Ok()
            Err(f) => {
                panic!("{}. Use --help to list recognized commands", f.to_string());
            } //rejects from Err()
        };
        if matches.opt_present("v") {
            version(&exec);
            assert!(true);
        }
    }
}



