use ninjalib::ninjalib::NinjaFile;
use std::env;

struct AppConfig {
    filename: String,
    max_files: usize,
    is_exit: bool,
    is_gui: bool,
    is_sort_by_name: bool,
}

fn parge_args(args: Vec<String>) -> AppConfig {
    let mut name: String = "".to_string();
    let mut max_files: usize = 4096;
    let mut exit = false;
    let mut gui = false;
    let mut sort_name = false;

    let mut i = 0;
    while i < args.len() {
        if args[i].as_str() == "-f" {
            if (i + 1) < args.len() {
                name = args[i + 1].as_str().to_string();
                i += 1;
            }
        }
        if args[i].as_str() == "-m" {
            if (i + 1) < args.len() {
                max_files = args[i + 1].as_str().to_string().parse::<usize>().unwrap();
                i += 1;
            }
        }
        if args[i].as_str() == "--sort-by-name" {
            sort_name = true;
        }
        if args[i].as_str() == "--gui" {
            gui = true;
        }
        if args[i].as_str() == "-h" || args[i].as_str() == "--help" {
            exit = true;
            println!("Statistics of ninja log file");
            println!("args:");
            println!("    {:16} {}", "-h --help", "print this help message");
            println!("    {:16} {}", "-f <path>", "path to ninja log file");
            println!("    {:16} {}", "--gui", "run gui for view ninja log (default: false)");
            println!("    {:16} {}", "--sort-by-name", "sort stats by filename (default: by time)");
            println!("    {:16} {}", "-m <int>", "maximum lines of top slow files (default: 4096)");
        }

        i += 1;
    }

    AppConfig {
        filename: name,
        max_files: max_files,
        is_exit: exit,
        is_gui: gui,
        is_sort_by_name: sort_name,
    }
}

fn main() {
    let config = parge_args(env::args().collect());
    if config.is_exit {
        return;
    }

    let mut ninja = NinjaFile::new(config.filename.as_str());
    let str_stats = ninja.to_string(config.max_files, config.is_sort_by_name);
    println!("{}", str_stats);
}
