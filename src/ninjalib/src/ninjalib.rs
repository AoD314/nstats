use std::fmt::Write;
use std::fs::read_to_string;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct NinjaRecord {
    pub dur: u64,
    pub start: u64,
    pub end: u64,
    pub time: u64,
    pub thread_id: u32,
    pub cmd: String,
    pub hash: String,
}

pub struct NinjaStats {
    pub total_time: u64,
    pub sum_time: u64,
    pub files: u32,
    pub threads: u32,
}

pub struct NinjaFile {
    pub records: Vec<NinjaRecord>,
    pub stats: NinjaStats,
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
            self.records.sort_by_key(|a| std::cmp::Reverse(a.dur));
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
        writeln!(&mut str_stats, "  threads detect: {:?}", self.stats.threads).unwrap();

        str_stats
    }
}

fn parse_ninja_log(lines: Vec<String>) -> NinjaFile {
    let mut records = Vec::with_capacity(lines.len());

    let mut stats = NinjaStats {
        total_time: 0,
        sum_time: 0,
        files: 0,
        threads: 0,
    };

    for r in lines.iter() {
        let arr = r.split("\t").collect::<Vec<_>>();

        if arr.len() == 5 {
            let rec = NinjaRecord {
                start: arr[0].parse::<u64>().unwrap(),
                end: arr[1].parse::<u64>().unwrap(),
                time: arr[2].parse::<u64>().unwrap(),
                thread_id: 0,
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

    records.sort_by_key(|a| a.start);

    let mut threads_duration = Vec::with_capacity(records.len());
    threads_duration.push(0u64);

    for rec in records.iter_mut() {
        let s = rec.start;
        let f = rec.end;

        let mut thread_id = 0;
        let mut is_found = false;

        if let Some((index, _)) = threads_duration.iter().enumerate().filter(|(_, value)| s >= **value).min_by(|(_, a), (_, b)| a.cmp(b)) {
            is_found = true;
            thread_id = index;
        }

        if !is_found {
            threads_duration.push(0u64);
            thread_id = threads_duration.len() - 1;
        }

        threads_duration[thread_id] = f;

        rec.thread_id = thread_id as u32;
    }
    stats.threads = threads_duration.len() as u32;

    NinjaFile { records: records, stats: stats }
}
