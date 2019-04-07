extern crate rand;

use rand::Rng;
use std::fmt;

// OLD_YIN: A broken line changing to a solid line.
const OLD_YIN: u32 = 6;

// YOUNG_YANG: A solid line.
const YOUNG_YANG: u32 = 7;

// YOUNG_YIN: A broken line.
const YOUNG_YIN: u32 = 8;

// OLD_YANG: A solid line changing to a broken line.
const OLD_YANG: u32 = 9;

enum LineType {
    OldYin,
    YoungYang,
    YoungYin,
    OldYang,
}

struct Line {
    ln_type: LineType,
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = match &self.ln_type {
            LineType::OldYin => "--X--",
            LineType::YoungYang => "-----",
            LineType::YoungYin => "-- --",
            LineType::OldYang => "--0--",
        };
        write!(f, "{}", string)
    }
}

fn main() {
    println!("\n");
    // Initialize the randomness
    let mut rng = rand::thread_rng();
    // Start the counter at six because we want six lines
    let mut counter = 6;

    // Count down and flip coins
    while counter != 0 {
        let mut flips = 3;
        let mut line_total = 0;

        while flips != 0 {
            let flip = rng.gen();
            if flip {
                // Heads
                line_total = line_total + 3;
            } else {
                // Tails
                line_total = line_total + 2;
            }
            flips = flips - 1;
        }

        let line = match line_total {
            OLD_YIN => Line { ln_type: LineType::OldYin },
            YOUNG_YANG => Line { ln_type: LineType::YoungYang },
            YOUNG_YIN => Line { ln_type: LineType::YoungYin },
            OLD_YANG => Line { ln_type: LineType::OldYang },
            _ => unimplemented!()
        };

        println!("{}", line);
        counter = counter - 1;
    }
    println!("\n");
    println!("http://www.akirarabelais.com/i/i.html");
}
