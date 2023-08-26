mod beauties;

use beauties::*;

fn main() {
    let mut heads_beauty = ConsistentBeauty::new(CoinFlip::Heads);
    let mut tails_beauty = ConsistentBeauty::new(CoinFlip::Tails);
    let mut changing_beauty = ChangingBeauty::new();
    let mut random_beauty = RandomBeauty::new();

    let mut candidates: Vec<&mut dyn SleepingBeauty> = vec![
        &mut heads_beauty,
        &mut tails_beauty,
        &mut changing_beauty,
        &mut random_beauty,
    ];

    let num_trials = 100_000;

    run_experiments(&mut candidates, num_trials);
}

fn run_experiments<'a>(candidates: &mut Vec<&'a mut dyn SleepingBeauty>, num_trials: u32) {
    for candidate in candidates {
        let experiment_run = run_experiment(*candidate, num_trials);
        print_experiment_results(&experiment_run);
        println!();
    }
}

fn run_experiment<'a>(candiate: &'a mut dyn SleepingBeauty, num_trials: u32) -> ExperimentRun<'a> {
    let mut num_successes = 0;
    let mut num_failures = 0;
    let mut num_inconclusive = 0;

    for _ in 0..num_trials {
        // 1) Put sleeping beauty to bed
        candiate.prepare_for_trial();

        // 2) Flip coin
        let coin_flip = CoinFlip::random();

        // 3) If the coin was tails, wake sleeping beauty up on Monday and Tuesday
        //    and ask her to guess the coin flip.
        //    If the coin was heads, wake sleeping beauty up only on Monday
        //    and ask her to guess the coin flip.

        let mut correct_guesses = 0;
        let mut incorrect_guesses = 0;

        if coin_flip == CoinFlip::Tails {
            // Monday (Tails)
            match candiate.guess_coin_flip() {
                CoinFlip::Heads => incorrect_guesses += 1,
                CoinFlip::Tails => correct_guesses += 1,
            }

            // Tuesday (Tails)
            match candiate.guess_coin_flip() {
                CoinFlip::Heads => incorrect_guesses += 1,
                CoinFlip::Tails => correct_guesses += 1,
            }
        } else {
            // Monday (Heads)
            match candiate.guess_coin_flip() {
                CoinFlip::Heads => correct_guesses += 1,
                CoinFlip::Tails => incorrect_guesses += 1,
            }
        }

        match (correct_guesses > 0, incorrect_guesses > 0) {
            (true, true) => num_inconclusive += 1,
            (true, false) => num_successes += 1,
            (false, true) => num_failures += 1,
            (false, false) => unreachable!(),
        }
    }

    ExperimentRun {
        candiate,
        num_trials,
        num_successes,
        num_failures,
        num_inconclusive,
    }
}

fn print_experiment_results(experiment_run: &ExperimentRun) {
    println!("Candidate: {}", experiment_run.candiate.name());
    println!("Number of trials: {}", experiment_run.num_trials);
    println!("Number of successes: {}", experiment_run.num_successes);
    println!("Number of failures: {}", experiment_run.num_failures);
    println!(
        "Number of inconclusive results: {}",
        experiment_run.num_inconclusive
    );

    println!(
        "Success rate: {:.2}%",
        experiment_run.success_rate() * 100.0
    );
    println!(
        "Failure rate: {:.2}%",
        experiment_run.failure_rate() * 100.0
    );
    println!(
        "Inconclusive rate: {:.2}%",
        experiment_run.inconclusive_rate() * 100.0
    );
}

struct ExperimentRun<'a> {
    candiate: &'a dyn SleepingBeauty,
    num_trials: u32,
    num_successes: u32,
    num_failures: u32,
    num_inconclusive: u32,
}

impl<'a> ExperimentRun<'a> {
    fn success_rate(&self) -> f64 {
        self.num_successes as f64 / self.num_trials as f64
    }

    fn failure_rate(&self) -> f64 {
        self.num_failures as f64 / self.num_trials as f64
    }

    fn inconclusive_rate(&self) -> f64 {
        self.num_inconclusive as f64 / self.num_trials as f64
    }
}

pub trait SleepingBeauty {
    fn name(&self) -> &str;

    fn prepare_for_trial(&mut self) {}

    fn guess_coin_flip(&self) -> CoinFlip;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CoinFlip {
    Heads,
    Tails,
}

impl CoinFlip {
    fn random() -> CoinFlip {
        if rand::random() {
            CoinFlip::Heads
        } else {
            CoinFlip::Tails
        }
    }
}
