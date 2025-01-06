use std::fs::read_to_string;
use std::env;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct NinjaRecord {
    dur: u64,
    start: u64,
    end: u64,
    time: u64,
    cmd: String,
    hash: String,
    ext: String
}

struct AppConfig {
    filename: String,
    counttop: u64,
    is_exit: bool
}

struct NinjaStats {
    total_time: u64,
    sum_time: u64,
    files: u32
}

fn time_to_string(t: u64) -> String {
    let mut ms = t;
    let h = ms / (3600 * 1000);
    ms -= h * (3600 * 1000);
    let m = ms / (60 * 1000);
    ms -= m * (60 * 1000);
    let s = ms / (1 * 1000);
    ms -= s * (1 * 1000);

    format!("{:02}:{:02}:{:02}.{:03} ({:12} ms)", h, m, s, ms, t)
}

fn load_ninja_log(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn parse_ninja_log(lines: Vec<String>) -> Vec<NinjaRecord> {
    // let extensions = ["o", "link", "cpp", "c", "h", "hpp", "so", "a"];
    let mut records = Vec::new();

    for r in lines.iter() {
        let arr = r.split("\t").collect::<Vec<_>>();

        if arr.len() == 5 {
            let p = arr[3].to_string();

            let paths = p.split("/").collect::<Vec<_>>();
            let name = paths[paths.len() - 1];
            let mut extension = "link".to_string();

            let names_arr = name.split(".").collect::<Vec<_>>();

            if names_arr.len() > 1 {
                extension = names_arr[names_arr.len() - 1].to_string();
            }

            let rec = NinjaRecord {
                start: arr[0].parse::<u64>().unwrap(),
                end: arr[1].parse::<u64>().unwrap(),
                time: arr[2].parse::<u64>().unwrap(),
                cmd: paths.join("/").to_string(),
                hash: arr[4].to_string(),
                dur: arr[1].parse::<u64>().unwrap() - arr[0].parse::<u64>().unwrap(),
                ext: extension
            };
            records.push(rec);
        }
    }

    records
}

fn parge_args(args: Vec<String>) -> AppConfig {
    let mut name: String = "".to_string();
    let mut top: u64 = 1 << 63;
    let mut exit = false;

    let mut i = 0;
    while i < args.len() {
        if args[i].as_str() == "-f" {
            if (i + 1) < args.len() {
                name = args[i+1].as_str().to_string();
                i += 1;
            }
        }
        if args[i].as_str() == "-t" {
            if (i + 1) < args.len() {
                top = args[i+1].as_str().to_string().parse::<u64>().unwrap();
                i += 1;
            }
        }
        if args[i].as_str() == "-h" {
            exit = true;
            println!("Statistics of ninja log file");
            println!("args:");
            println!("\t-h\t \tprint this help message");
            println!("\t-f\t<path>\tpath to ninja log file");
            println!("\t-t\t<int>\tamount print lines of top slowly files");
        }
        i += 1;
    }

    return AppConfig {
        filename: name,
        counttop: top,
        is_exit: exit
    }
}

fn main() {
    let config = parge_args(env::args().collect());
    if config.is_exit {
        return;
    }

    let lines = load_ninja_log(&config.filename);
    let mut records = parse_ninja_log(lines);
    records.sort_by(|a, b| b.dur.cmp(&a.dur));

    let mut stats = NinjaStats {
        total_time: 0,
        sum_time: 0,
        files: 0
    };

    let mut count = 0;
    for r in &records {
        stats.sum_time += r.dur;
        stats.total_time = if stats.total_time < r.end { r.end } else { stats.total_time };
        stats.files += 1;
        count += 1;
        if count >= config.counttop {
            if count == config.counttop {
                println!("{:>8}", "...");
            }
        } else {
            println!("{:8} ms |    {}", r.dur, r.cmd);
        }
    }

    let speed_ratio = stats.sum_time as f64 / stats.total_time as f64;

    println!("\nStats:");
    println!("  cpu time      : {:?}", time_to_string(stats.sum_time));
    println!("  compile time  : {:?}", time_to_string(stats.total_time));
    println!("  speed ratio   : {:.2}", speed_ratio);
    println!("  avg build time: {:.1} ms", stats.sum_time as f64 / stats.files  as f64);
    println!("  files         : {:?}", stats.files);
    println!("  files per secs: {:.2}", stats.files  as f64 / (stats.total_time  as f64 / 1000.0));

}
