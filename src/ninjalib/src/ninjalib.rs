use std::fmt::Write;
use std::fs::read_to_string;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct NinjaRecord {
    dur: u64,
    start: u64,
    end: u64,
    time: u64,
    cmd: String,
    hash: String,
}

pub struct NinjaStats {
    total_time: u64,
    sum_time: u64,
    files: u32,
}

pub struct NinjaFile {
    records: Vec<NinjaRecord>,
    stats: NinjaStats,
}

pub fn time_to_string(t: u64) -> String {
    let mut ms = t;
    let h = ms / (3600 * 1000);
    ms -= h * (3600 * 1000);
    let m = ms / (60 * 1000);
    ms -= m * (60 * 1000);
    let s = ms / (1 * 1000);
    ms -= s * (1 * 1000);

    format!("{:02}:{:02}:{:02}.{:03} ({:12} ms)", h, m, s, ms, t)
}

impl NinjaFile {
    pub fn new(filename: &str) -> Self {
        let lines = read_to_string(filename).unwrap().lines().map(String::from).collect();
        parse_ninja_log(lines)
    }

    pub fn to_string(&mut self, max_files: usize, sort_by_name: bool) -> String {
        if sort_by_name {
            self.records.sort_by(|a, b| b.cmd.cmp(&a.cmd));
        } else {
            self.records.sort_by(|a, b| b.dur.cmp(&a.dur));
        }

        let cap = if max_files == 0 { 8192 } else { max_files * 128 };

        let mut str_stats = String::with_capacity(cap);

        for (i, rec) in self.records.iter().enumerate() {
            if i >= max_files {
                writeln!(&mut str_stats, "{:>8}", "...").unwrap();
                break;
            }
            writeln!(&mut str_stats, "{:8} ms |    {}", rec.dur, rec.cmd).unwrap();
        }

        let speed_ratio = self.stats.sum_time as f64 / self.stats.total_time as f64;

        writeln!(&mut str_stats, "\nStats:").unwrap();
        writeln!(&mut str_stats, "  cpu time (1T) : {}", time_to_string(self.stats.sum_time)).unwrap();
        writeln!(&mut str_stats, "  compile time  : {}", time_to_string(self.stats.total_time)).unwrap();
        writeln!(&mut str_stats, "  speed ratio   : {:.2}", speed_ratio).unwrap();
        writeln!(&mut str_stats, "  avg build time: {:.2} ms", self.stats.sum_time as f64 / self.stats.files as f64).unwrap();
        writeln!(&mut str_stats, "  files per secs: {:.2}", self.stats.files as f64 / (self.stats.total_time as f64 / 1000.0)).unwrap();
        writeln!(&mut str_stats, "  files         : {:?}", self.stats.files).unwrap();

        str_stats
    }
}

fn parse_ninja_log(lines: Vec<String>) -> NinjaFile {
    let mut records = Vec::with_capacity(lines.len());

    let mut stats = NinjaStats {
        total_time: 0,
        sum_time: 0,
        files: 0,
    };

    for r in lines.iter() {
        let arr = r.split("\t").collect::<Vec<_>>();

        if arr.len() == 5 {
            let rec = NinjaRecord {
                start: arr[0].parse::<u64>().unwrap(),
                end: arr[1].parse::<u64>().unwrap(),
                time: arr[2].parse::<u64>().unwrap(),
                cmd: arr[3].to_string(),
                hash: arr[4].to_string(),
                dur: arr[1].parse::<u64>().unwrap() - arr[0].parse::<u64>().unwrap(),
            };

            stats.files += 1;
            stats.sum_time += rec.dur;
            stats.total_time = if stats.total_time < rec.end { rec.end } else { stats.total_time };

            records.push(rec);
        }
    }

    NinjaFile { records: records, stats: stats }
}
