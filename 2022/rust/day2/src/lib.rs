#[derive(Clone, Copy, PartialEq)]
pub enum RPSChoice {
    Rock,
    Paper,
    Scissors,
}

impl RPSChoice {
    pub fn new(val: char) -> Self {
        match val {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Scissors,
            _ => panic!("bad value"),
        }
    }

    fn value(self) -> u8 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn outcome(self, theirs: Self) -> u8 {
        if self == theirs {
            3
        } else if self == RPSChoice::Rock {
            if theirs == RPSChoice::Scissors {
                6
            } else {
                0
            }
        } else if self == RPSChoice::Paper {
            if theirs == RPSChoice::Rock {
                6
            } else {
                0
            }
        } else {
            if theirs == RPSChoice::Paper {
                6
            } else {
                0
            }
        }
    }

    fn get_loser(self) -> RPSChoice {
        match self {
            RPSChoice::Rock => RPSChoice::Paper,
            RPSChoice::Paper => RPSChoice::Scissors,
            RPSChoice::Scissors => RPSChoice::Rock,
        }
    }

    fn get_winner(self) -> RPSChoice {
        match self {
            RPSChoice::Rock => RPSChoice::Scissors,
            RPSChoice::Paper => RPSChoice::Rock,
            RPSChoice::Scissors => RPSChoice::Paper,
        }
    }

}

#[derive(Clone, Copy)]
pub enum DesiredOutcome {
    Lose,
    Draw,
    Win,
}

impl DesiredOutcome {
    pub fn new(val: char) -> Self {
        match val {
            'X' => Self::Lose,
            'Y' => Self::Draw,
            'Z' => Self::Win,
            _ => panic!("Bad input DesiredOutcome"),
        }
    }

    pub fn get_rps_choice(self, theirs: RPSChoice) -> RPSChoice {
        match self {
            Self::Lose => theirs.get_winner(),
            Self::Win => theirs.get_loser(),
            Self::Draw => theirs,
        }
    }
}

pub fn get_score(theirs: RPSChoice, mine: RPSChoice) -> u8 {
    mine.value() + mine.outcome(theirs) 
}

