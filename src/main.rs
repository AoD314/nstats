use std::collections::HashMap;
use std::fs::read_to_string;
use std::env;
use std::collections::HashSet;
use std::u64::{MAX, MIN};

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

#[derive(Debug,Hash)]
struct GroupStats {
    sum: u64,
    min: u64,
    max: u64,
    count: u32,
    filename_min: String,
    filename_max: String
}
impl GroupStats {
    fn new() -> GroupStats {
        GroupStats {
            sum: 0,
            min: MAX,
            max: MIN,
            count: 0,
            filename_min: String::new(),
            filename_max: String::new()
        }
    }
}

struct AppConfig {
    filename: String,
    counttop: u64,
    is_group: bool,
    is_exit: bool,
    is_sort_name: bool
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
    let mut records = Vec::with_capacity(100);

    for r in lines.iter() {
        let arr = r.split("\t").collect::<Vec<_>>();

        if arr.len() == 5 {
            let p = arr[3].to_string();

            let paths = p.split("/").collect::<Vec<_>>();
            let name = paths[paths.len() - 1];
            let mut extension = "app".to_string();

            let names_arr = name.split(".").collect::<Vec<_>>();

            if names_arr.len() > 1 {
                extension = names_arr[names_arr.len() - 1].to_string();
            }

            let rec = NinjaRecord {
                start: arr[0].parse::<u64>().unwrap(),
                end: arr[1].parse::<u64>().unwrap(),
                time: arr[2].parse::<u64>().unwrap(),
                cmd: p,
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
    let mut top: u64 = MAX;
    let mut group: bool = false;
    let mut exit = false;
    let mut sort_name = false;

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
        if args[i].as_str() == "-n" || args[i].as_str() == "--sort-name" {
            sort_name = true;
        }
        if args[i].as_str() == "-h" || args[i].as_str() == "--help" {
            exit = true;
            println!("Statistics of ninja log file");
            println!("args:");
            println!("    {:16} {}", "-h --help", "print this help message");
            println!("    {:16} {}", "-f <path>", "path to ninja log file");
            println!("    {:16} {}", "-g", "print group stats");
            println!("    {:16} {}", "-n --sort-name", "sort stats by filename");
            println!("    {:16} {}", "-t <int>", "amount print lines of top slowly files");
        }
        if args[i].as_str() == "-g" {
            group = true;
        }

        i += 1;
    }

    return AppConfig {
        filename: name,
        counttop: top,
        is_group: group,
        is_exit: exit,
        is_sort_name: sort_name
    }
}

fn main() {
    let config = parge_args(env::args().collect());
    if config.is_exit {
        return;
    }

    let lines = load_ninja_log(&config.filename);
    let mut records = parse_ninja_log(lines);
    if config.is_sort_name {
        records.sort_by(|a, b| b.cmd.cmp(&a.cmd));
    }
    else {
        records.sort_by(|a, b| b.dur.cmp(&a.dur));
    }


    let mut stats = NinjaStats {
        total_time: 0,
        sum_time: 0,
        files: 0
    };

    let extensions = HashSet::from(["o", "so", "app", "cpp"]);
    let mut group_stats_map = HashMap::new();
    for ext in &extensions {
        let gs = GroupStats::new();
        group_stats_map.insert(ext, gs);
    };

    let mut count = 0;
    for r in &records {
        stats.sum_time += r.dur;
        stats.total_time = if stats.total_time < r.end { r.end } else { stats.total_time };
        stats.files += 1;

        if extensions.contains(r.ext.as_str()) {
            match group_stats_map.get_mut(&r.ext.as_str()) {
                Some(gs) => {
                    gs.count += 1;
                    if gs.max < r.dur {
                        gs.max = r.dur;
                        gs.filename_max = r.cmd.clone();
                    }
                    if gs.min > r.dur {
                        gs.min = r.dur;
                        gs.filename_min = r.cmd.clone();
                    }
                    gs.sum += r.dur;
                },
                None => (),
            }
        }

        count += 1;
        if count >= config.counttop {
            if count == config.counttop {
                println!("{:>8}", "...");
            }
        } else {
            println!("{:8} ms |    {}", r.dur, r.cmd);
        }
    }

    if config.is_group {
        println!("GroupStats:");
        for (ext, gs) in &group_stats_map {
            println!("{ext}: {}", gs.count);
            println!("\t{:8}: {}", "sum", time_to_string(gs.sum));
            println!("\t{:8}: {} ms ({})", "min", gs.min, gs.filename_min);
            println!("\t{:8}: {} ms ({})", "max", gs.max, gs.filename_max);
            println!("\t{:8}: {:.1} ms", "avg", gs.sum as f64 / gs.count as f64);
        }
    }

    let speed_ratio = stats.sum_time as f64 / stats.total_time as f64;

    println!("\nStats:");
    println!("  cpu time (1T) : {:?}", time_to_string(stats.sum_time));
    println!("  compile time  : {:?}", time_to_string(stats.total_time));
    println!("  speed ratio   : {:.2}", speed_ratio);
    println!("  avg build time: {:.1} ms", stats.sum_time as f64 / stats.files  as f64);
    println!("  files         : {:?}", stats.files);
    println!("  files per secs: {:.2}", stats.files  as f64 / (stats.total_time  as f64 / 1000.0));

}
