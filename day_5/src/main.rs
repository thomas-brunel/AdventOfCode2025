// From https://adventofcode.com/2025/day/5

// --- Day 5: Cafeteria ---

// As the forklifts break through the wall, the Elves are delighted to discover
// that there was a cafeteria on the other side after all.

// You can hear a commotion coming from the kitchen. "At this rate, we won't
// have any time left to put the wreaths up in the dining hall!" Resolute in
// your quest, you investigate.

// "If only we hadn't switched to the new inventory management system right
// before Christmas!" another Elf exclaims. You ask what's going on.

// The Elves in the kitchen explain the situation: because of their complicated
// new inventory management system, they can't figure out which of their
// ingredients are fresh and which are spoiled. When you ask how it works, they
// give you a copy of their database (your puzzle input).

// The database operates on ingredient IDs. It consists of a list of fresh
// ingredient ID ranges, a blank line, and a list of available ingredient IDs.
// For example:

// 3-5
// 10-14
// 16-20
// 12-18

// 1
// 5
// 8
// 11
// 17
// 32

// The fresh ID ranges are inclusive: the range 3-5 means that ingredient
// IDs 3, 4, and 5 are all fresh. The ranges can also overlap; an ingredient ID
// is fresh if it is in any range.

// The Elves are trying to determine which of the available ingredient IDs are
// fresh. In this example, this is done as follows:

//    Ingredient ID 1 is spoiled because it does not fall into any range.
//    Ingredient ID 5 is fresh because it falls into range 3-5.
//    Ingredient ID 8 is spoiled.
//    Ingredient ID 11 is fresh because it falls into range 10-14.
//    Ingredient ID 17 is fresh because it falls into range 16-20 as well as
//    range 12-18.
//    Ingredient ID 32 is spoiled.

// So, in this example, 3 of the available ingredient IDs are fresh.

// Process the database file from the new inventory management system. How many
// of the available ingredient IDs are fresh?

// --- Part Two ---

// The Elves start bringing their spoiled inventory to the trash chute at the
// back of the kitchen.

// So that they can stop bugging you when they get new inventory, the Elves
// would like to know all of the IDs that the fresh ingredient ID ranges
// consider to be fresh. An ingredient ID is still considered fresh if it is in
// any range.

// Now, the second section of the database (the available ingredient IDs) is
// irrelevant. Here are the fresh ingredient ID ranges from the above example:

// 3-5
// 10-14
// 16-20
// 12-18

// The ingredient IDs that these ranges consider to be fresh
// are 3, 4, 5, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, and 20.
// So, in this example, the fresh ingredient ID ranges consider a total of 14
// ingredient IDs to be fresh.

// Process the database file again. How many ingredient IDs are considered to be
// fresh according to the fresh ingredient ID ranges?

use std::io::BufRead;

fn main() {
    let mut p1_total_of_fresh_ingredient = 0;
    let mut p2_total_of_fresh_ingredient = 0;
    let mut reading_fresh_ingredient_ranges: bool = true;
    let mut fresh_ingredient_ranges: Vec<(u64, u64)> = vec![];

    let Ok(file) = std::fs::File::open("input.txt") else {
        panic!("Failed to open file: `input.txt`");
    };

    let reader = std::io::BufReader::new(file);
    for line in reader.lines().map(|l| l.unwrap()) {
        if line.is_empty() {
            println!(
                "This is the blank line separating fresh ingredient ID ranges from the list of available ingredient IDs."
            );
            reading_fresh_ingredient_ranges = false;
            continue;
        }
        if reading_fresh_ingredient_ranges {
            let Some((lower_range, upper_range)) = line.split_once("-") else {
                panic!("Failed to split range of line {}", line);
            };
            let Ok(lower_range) = lower_range.parse() else {
                panic!("Failed to convert lower range {}", lower_range);
            };
            let Ok(upper_range) = upper_range.parse() else {
                panic!("Failed to convert upper range {}", upper_range);
            };
            fresh_ingredient_ranges.push((lower_range, upper_range));
        } else {
            let Ok(value) = line.parse::<u64>() else {
                panic!("Failed to convert ingredient for line {line}");
            };
            let mut is_fresh: bool = false;
            for (lower_range, upper_range) in &fresh_ingredient_ranges {
                if value >= *lower_range && value <= *upper_range {
                    println!(
                        "Ingredient ID {value} is fresh, falls into range {lower_range}-{upper_range}"
                    );
                    is_fresh = true;
                    break;
                }
            }
            if is_fresh {
                p1_total_of_fresh_ingredient += 1;
            } else {
                println!("Ingredient ID {value} is spoiled!");
            }
        }
    }

    fresh_ingredient_ranges.sort_by(|a, b| a.0.cmp(&b.0));
    let mut merged_fresh_ingredient_ranges: Vec<(u64, u64)> = vec![];
    for (lower_range, upper_range) in fresh_ingredient_ranges {
        let mut merged: bool = false;
        for (lw_range, up_range) in &mut merged_fresh_ingredient_ranges {
            if *lw_range >= lower_range && *lw_range <= upper_range {
                if *up_range <= upper_range {
                    *up_range = upper_range;
                }
                *lw_range = lower_range;
                merged = true;
            } else if *lw_range <= lower_range && *up_range >= lower_range {
                if *up_range <= upper_range {
                    *up_range = upper_range;
                }
                merged = true;
            }
        }
        if !merged {
            merged_fresh_ingredient_ranges.push((lower_range, upper_range));
        }
    }

    merged_fresh_ingredient_ranges.iter().for_each(|(lw, up)| {
        println!("Merged range {lw}-{up}");
        p2_total_of_fresh_ingredient += up - lw + 1;
    });

    println!(
        "[PART 1] Total of fresh ingredient: {}",
        p1_total_of_fresh_ingredient
    );
    println!(
        "[PART 2] Total of fresh ingredient based of the ranges of database: {}",
        p2_total_of_fresh_ingredient
    );
}
