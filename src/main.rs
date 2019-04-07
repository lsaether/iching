extern crate rand;

use rand::Rng;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io;

// OLD_YIN: A broken line changing to a solid line.
const OLD_YIN: u8 = 6;

// YOUNG_YANG: A solid line.
const YOUNG_YANG: u8 = 7;

// YOUNG_YIN: A broken line.
const YOUNG_YIN: u8 = 8;

// OLD_YANG: A solid line changing to a broken line.
const OLD_YANG: u8 = 9;

#[derive(Debug)]
struct Line {
    total: u8,
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = match self.total {
            OLD_YIN     => "--X--",
            YOUNG_YANG  => "-----",
            YOUNG_YIN   => "-- --",
            OLD_YANG    => "--0--",
            _           => unimplemented!(),
        };
        write!(f, "{}", string)
    }
}   

#[derive(Debug)]
struct Trigram {
    lines: Vec<Line>,
}

impl Trigram {
    fn get_name(&self) -> usize {
        if &self.lines[0].total % 2 == 1 {
            if &self.lines[1].total % 2 == 1 {
                if &self.lines[2].total % 2 == 1 {
                    0 // Chien
                } else {
                    // Third line is broken
                    5 // Sun
                }
            } else {
                // Second line is broken
                if &self.lines[2].total % 2 == 1 {
                    // Third line is solid
                    6 // Li
                } else {
                    // Third line is broken
                    3 // Ken
                }
            }
        } else {
            // First line is broken
            if &self.lines[1].total % 2 == 1 {
                // Second line is solid
                if &self.lines[2].total % 2 == 1 {
                    // Third line is solid
                    7 // Tui
                } else {
                    // Third line is broken
                    2 // Kan
                }
            } else {
                // Second line is broken
                if &self.lines[2].total % 2 == 1 {
                    // Third line is solid
                    1 // Chen
                } else {
                    // They're all broken
                    4 // Kun
                }
            }
        }
    }
}

#[derive(Debug)]
struct Hexagram {
    below: Trigram,
    above: Trigram,
}

impl Hexagram {
    fn print(&self) {
        for line in &self.above.lines {
            println!("{}", line);
        }
        for line in &self.below.lines {
            println!("{}", line);
        }
    }

    fn reading(&self) {
        let chien_row = vec![1, 34, 5, 26, 11, 9, 14, 43];
        let chen_row = vec![25, 51, 3, 27, 24, 42, 21, 17];
        let kan_row = vec![6, 40, 29, 4, 7, 59, 64, 47];
        let ken_row = vec![33, 62, 39, 52, 15, 53, 56, 31];
        let kun_row = vec![12, 16, 8, 23, 2, 20, 35, 45];
        let sun_row = vec![44, 32, 48, 18, 46, 57, 50, 28];
        let li_row = vec![13, 55, 63, 22, 36, 37, 30, 49];
        let tui_row = vec![10, 54, 60, 41, 19, 61, 38, 58];
        let rows = vec![
            chien_row,
            chen_row,
            kan_row,
            ken_row,
            kun_row,
            sun_row,
            li_row,
            tui_row,
        ];

        let lower = &self.below.get_name();
        let upper = &self.above.get_name();
        let reading_index = rows[*lower][*upper];

        read_file(reading_index);
    }
}

fn read_file(index: u8){
    let contents = include_str!("./i.txt");
    let search_string = format!("<a name=\"{}\"></a>", index);
    let end_string = "<a href=\"#index\">index</a><br><br>";
    // println!("{}", contents);
    let found = contents.find(&search_string).unwrap();
    let array = contents.split_at(found);

    let array2: Vec<&str> = array.1.split(&end_string).collect();
    let result = array2[1];
    println!("{}", result);

}

fn main() {
    println!("\n");
    // Initialize the randomness
    let mut rng = rand::thread_rng();
    // Start the counter at six because we want six lines
    let mut counter = 6;

    let mut below = vec![];
    let mut above = vec![];

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

        let line = Line { total: line_total };

        if counter > 3 {
            below.push(line);
        } else {
            above.push(line);
        }

        counter = counter - 1;
    }

    let hexagram = Hexagram {
        below: Trigram { lines: below },
        above: Trigram { lines: above },
    };

    hexagram.print();

    println!("\nShow reading? (Y/n)");
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        if line.unwrap() == "n" {
            println!("\nhttp://www.akirarabelais.com/i/i.html");
        } else {
            hexagram.reading();
        }
        ::std::process::exit(0);
    }
}
