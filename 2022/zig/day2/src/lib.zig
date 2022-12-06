const std = @import("std");

pub fn get_score(mine: RPSChoice, theirs: RPSChoice) u8 {
    return mine.value() + mine.outcome(theirs);
}

pub const RPSChoice = enum {
    Rock,
    Paper,
    Scissors,

    pub fn new(val: u8) RPSChoice {
        const rc = switch (val) {
            'A', 'X' => RPSChoice.Rock,
            'B', 'Y' => RPSChoice.Paper,
            'C', 'Z' => RPSChoice.Scissors,
            else => @panic("oh no"),
        };

        return rc;
    }

    pub fn value(self: RPSChoice) u8 {
        const rc: u8 = switch (self) {
            RPSChoice.Rock => 1,
            RPSChoice.Paper => 2,
            RPSChoice.Scissors => 3,
        };

        return rc;
    }

    pub fn outcome(self: RPSChoice, theirs: RPSChoice) u8 {
        if (self == theirs) {
            return 3;
        }

        var rc = switch (self) {
            RPSChoice.Rock => if (theirs == RPSChoice.Scissors) {
                return 6;
            } else {
                return 0;
            },
            RPSChoice.Paper => if (theirs == RPSChoice.Rock) {
                return 6;
            } else {
                return 0;
            },
            RPSChoice.Scissors => if (theirs == RPSChoice.Paper) {
                return 6;
            } else {
                return 0;
            },
        };

        return rc;
    }

    pub fn get_winner(self: RPSChoice) RPSChoice {
        const rc = switch (self) {
            RPSChoice.Rock => RPSChoice.Scissors,
            RPSChoice.Paper => RPSChoice.Rock,
            RPSChoice.Scissors => RPSChoice.Paper,
        };

        return rc;
    }

    pub fn get_loser(self: RPSChoice) RPSChoice {
        const rc = switch (self) {
            RPSChoice.Rock => RPSChoice.Paper,
            RPSChoice.Paper => RPSChoice.Scissors,
            RPSChoice.Scissors => RPSChoice.Rock,
        };

        return rc;
    }
};

pub const DesiredOutcome = enum {
    Loss,
    Draw,
    Win,

    pub fn new(val: u8) DesiredOutcome {
        const rc = switch (val) {
            'X' => DesiredOutcome.Loss,
            'Y' => DesiredOutcome.Draw,
            'Z' => DesiredOutcome.Win,
            else => @panic("oh no"),
        };

        return rc;
    }

    pub fn get_choice(self: DesiredOutcome, theirs: RPSChoice) RPSChoice {
        const rc = switch (self) {
            DesiredOutcome.Loss => theirs.get_winner(),
            DesiredOutcome.Win => theirs.get_loser(),
            DesiredOutcome.Draw => theirs,
        };

        return rc;
    }
};
