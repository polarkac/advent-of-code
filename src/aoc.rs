use std::time::{Duration, Instant};

use crate::cli::Args;

type FnInput = Box<dyn Fn() -> &'static str>;
type FnPart = Box<dyn Fn(&str) -> String>;

pub struct Aoc<'a> {
    args: &'a Args,
    input: FnInput,
    parts: Vec<FnPart>,
}

impl<'a> Aoc<'a> {
    pub fn new(args: &'a Args, input: FnInput) -> Self {
        Self {
            args,
            input,
            parts: Vec::new(),
        }
    }

    pub fn part(mut self, part: FnPart) -> Self {
        self.parts.push(part);

        self
    }

    pub fn run(&self) {
        let input = (self.input)();
        self.run_parts(input);
    }

    pub fn run_benchmarked(&self) {
        let input = (self.input)();
        let mut part_times = Vec::with_capacity(self.parts.len());
        for part in &self.parts {
            let mut times = Vec::with_capacity(25);
            while times.len() < 25 {
                let time = Instant::now();
                part(input);
                times.push(time.elapsed());
            }
            part_times.push(times);
        }

        self.display_benchmark_times(part_times);
    }

    fn run_parts(&self, input: &str) {
        for (i, part) in self.parts.iter().enumerate() {
            self.display_title(i + 1);
            let result = part(input);
            if i != self.parts.len() - 1 {
                println!("{result}\n");
            } else {
                println!("{result}");
            }
        }
    }

    fn display_title(&self, part_num: usize) {
        println!(
            "= {}/{} - part {} ==",
            self.args.year.as_u16(),
            self.args.day.as_u8(),
            part_num
        );
    }

    fn display_benchmark_times(&self, part_times: Vec<Vec<Duration>>) {
        for (i, part) in part_times.iter().enumerate() {
            self.display_title(i + 1);
            let min = part
                .iter()
                .map(|d| d.as_secs_f64())
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap();
            let max = part
                .iter()
                .map(|d| d.as_secs_f64())
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap();
            let avg = part.iter().map(|d| d.as_secs_f64()).sum::<f64>() / part.len() as f64;
            println!("    Average:\t {}", humanize_time(avg));
            let min_max = format!(
                "    Min … Max:\t {} … {}",
                humanize_time(min),
                humanize_time(max)
            );
            if i != part_times.len() - 1 {
                println!("{}\n", min_max);
            } else {
                println!("{}", min_max);
            }
        }
    }
}

fn humanize_time(value: f64) -> String {
    let units = ["s", "ms", "μs", "ns"];
    let (mut value, mut unit) = (value, units[0]);
    for (i, u) in units.iter().skip(1).enumerate() {
        if value < 1.0 {
            value = value * (1000.0 * (i + 1) as f64);
            unit = u;
        }
    }

    format!("{value:.2} {unit}")
}
