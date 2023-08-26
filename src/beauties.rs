use crate::{CoinFlip, SleepingBeauty};

pub struct ConsistentBeauty {
    name: String,
    guess: CoinFlip,
}

impl ConsistentBeauty {
    pub fn new(guess: CoinFlip) -> Self {
        Self {
            name: format!("Consistent Beauty ({:?})", guess),
            guess,
        }
    }
}

impl SleepingBeauty for ConsistentBeauty {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn guess_coin_flip(&self) -> CoinFlip {
        self.guess
    }
}

pub struct ChangingBeauty {
    name: String,
    guess: CoinFlip,
}

impl ChangingBeauty {
    pub fn new() -> Self {
        Self {
            name: "Changing Beauty".to_string(),
            guess: CoinFlip::Heads,
        }
    }
}

impl SleepingBeauty for ChangingBeauty {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn prepare_for_trial(&mut self) {
        self.guess = CoinFlip::random();
    }

    fn guess_coin_flip(&self) -> CoinFlip {
        self.guess
    }
}

pub struct RandomBeauty {
    name: String,
}

impl RandomBeauty {
    pub fn new() -> Self {
        Self {
            name: "Random Beauty".to_string(),
        }
    }
}

impl SleepingBeauty for RandomBeauty {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn guess_coin_flip(&self) -> CoinFlip {
        CoinFlip::random()
    }
}
