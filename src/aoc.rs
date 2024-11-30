use std::time::Instant;

type FnInput = Box<dyn Fn() -> &'static str>;
type FnPart = Box<dyn Fn(&str) -> String>;

pub struct Aoc {
    input: FnInput,
    parts: Vec<FnPart>,
}

impl Aoc {
    pub fn new(input: FnInput) -> Self {
        Self {
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
                times.push((Instant::now() - time).as_secs_f64());
            }
            part_times.push(times);
        }

        Self::display_benchmark_times(part_times);
    }

    fn run_parts(&self, input: &str) {
        for (i, part) in self.parts.iter().enumerate() {
            println!("= Part {} =", i + 1);
            let result = part(input);
            println!("{result}");
        }
    }

    fn display_benchmark_times(part_times: Vec<Vec<f64>>) {
        for (i, part) in part_times.iter().enumerate() {
            println!("= Part {} =", i + 1);
            let min = part
                .iter()
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap();
            let max = part
                .iter()
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap();
            let avg = part.iter().sum::<f64>() / part.len() as f64;
            println!("Min\tMax\tAvg");
            println!("{min} s\t{max} s\t{avg} s");
        }
    }
}
