// From https://adventofcode.com/2025/day/2

// --- Day 2: Gift Shop ---

// You get inside and take the elevator to its only other stop: the gift shop.
// "Thank you for visiting the North Pole!" gleefully exclaims a nearby sign.
// You aren't sure who is even allowed to visit the North Pole, but you know you
// can access the lobby through here, and from there you can access the rest of
// the North Pole base.

// As you make your way through the surprisingly extensive selection, one of the
// clerks recognizes you and asks for your help.

// As it turns out, one of the younger Elves was playing on a gift shop computer
// and managed to add a whole bunch of invalid product IDs to their gift shop
// database! Surely, it would be no trouble for you to identify the invalid
// product IDs for them, right?

// They've even checked most of the product ID ranges already; they only have a
// few product ID ranges (your puzzle input) that you'll need to check.
// For example:

// 11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
// 1698522-1698528,446443-446449,38593856-38593862,565653-565659,
// 824824821-824824827,2121212118-2121212124

// (The ID ranges are wrapped here for legibility; in your input, they appear on
// a single long line.)

// The ranges are separated by commas (,); each range gives its first ID and
// last ID separated by a dash (-).

// Since the young Elf was just doing silly patterns, you can find the invalid
// IDs by looking for any ID which is made only of some sequence of digits
// repeated twice. So, 55 (5 twice), 6464 (64 twice), and 123123 (123 twice)
// would all be invalid IDs.

// None of the numbers have leading zeroes; 0101 isn't an ID at all.
// (101 is a valid ID that you would ignore.)

// Your job is to find all of the invalid IDs that appear in the given ranges.
// In the above example:

//    11-22 has two invalid IDs, 11 and 22.
//    95-115 has one invalid ID, 99.
//    998-1012 has one invalid ID, 1010.
//    1188511880-1188511890 has one invalid ID, 1188511885.
//    222220-222224 has one invalid ID, 222222.
//    1698522-1698528 contains no invalid IDs.
//    446443-446449 has one invalid ID, 446446.
//    38593856-38593862 has one invalid ID, 38593859.
//    The rest of the ranges contain no invalid IDs.

// Adding up all the invalid IDs in this example produces 1227775554.

// What do you get if you add up all of the invalid IDs?

// --- Part Two ---

// The clerk quickly discovers that there are still invalid IDs in the ranges in
// your list. Maybe the young Elf was doing other silly patterns as well?

// Now, an ID is invalid if it is made only of some sequence of digits repeated
// at least twice. So, 12341234 (1234 two times), 123123123 (123 three times),
// 1212121212 (12 five times), and 1111111 (1 seven times) are all invalid IDs.

// From the same example as before:

//    11-22 still has two invalid IDs, 11 and 22.
//    95-115 now has two invalid IDs, 99 and 111.
//    998-1012 now has two invalid IDs, 999 and 1010.
//    1188511880-1188511890 still has one invalid ID, 1188511885.
//    222220-222224 still has one invalid ID, 222222.
//    1698522-1698528 still contains no invalid IDs.
//    446443-446449 still has one invalid ID, 446446.
//    38593856-38593862 still has one invalid ID, 38593859.
//    565653-565659 now has one invalid ID, 565656.
//    824824821-824824827 now has one invalid ID, 824824824.
//    2121212118-2121212124 now has one invalid ID, 2121212121.

// Adding up all the invalid IDs in this example produces 4174379265.

// What do you get if you add up all of the invalid IDs using these new rules?

use std::{fs, io::BufRead};

fn is_invalid_id_part1(value: &i64) -> bool {
    let mut is_invalid: bool = false;
    let number_of_digit: u32 = value.ilog10() + 1;
    // Check only numbers of even digits
    if number_of_digit % 2 == 0 {
        let divider: i64 = 10_i64.pow(number_of_digit / 2);
        let upper_number = value / divider;
        let lower_number = value - (upper_number * divider);
        is_invalid = (upper_number - lower_number) == 0;
    }
    is_invalid
}

fn is_invalid_id_part2(value: &i64) -> bool {
    let mut is_invalid: bool = false;
    let number_of_digits: u32 = value.ilog10() + 1;
    let str_value = value.to_string();
    for pattern_index in 1..number_of_digits {
        if (number_of_digits % pattern_index) != 0 {
            continue;
        }
        let (pattern, _) = str_value.split_at(pattern_index as usize);
        let reconstructed_value = pattern.repeat((number_of_digits / pattern_index) as usize);
        if reconstructed_value == str_value {
            is_invalid = true;
        }
    }

    is_invalid
}

fn main() {
    let mut p1_sum_of_invalid_ids = 0;
    let mut p2_sum_of_invalid_ids = 0;

    let Ok(file) = fs::File::open("input.txt") else {
        panic!("Failed to open file: `input.txt`");
    };

    let reader = std::io::BufReader::new(file);

    // Should have only 1 line
    for line in reader.lines().map(|l| l.unwrap()) {
        let ranges: Vec<&str> = line.split(',').collect();
        for range in ranges {
            let Some((lower_range, upper_range)) = range.split_once('-') else {
                panic!("Range found invalid! {}", range);
            };

            let numeric_lower_range: i64 = lower_range.parse().unwrap();
            let numeric_upper_range: i64 = upper_range.parse().unwrap();

            for value in numeric_lower_range..=numeric_upper_range {
                if value <= 0 {
                    continue;
                }
                if is_invalid_id_part1(&value) {
                    println!(
                        "[PART 1] Found invalid in range [{}-{}] - value {}",
                        numeric_lower_range, numeric_upper_range, value
                    );
                    p1_sum_of_invalid_ids += value;
                }
                if is_invalid_id_part2(&value) {
                    println!(
                        "[PART 2] Found invalid in range [{}-{}] - value {}",
                        numeric_lower_range, numeric_upper_range, value
                    );
                    p2_sum_of_invalid_ids += value;
                }
            }
        }
    }
    println!(
        "[PART 1] The sum of invalid ids produces {}",
        p1_sum_of_invalid_ids
    );
    println!(
        "[PART 2] The sum of invalid ids produces {}",
        p2_sum_of_invalid_ids
    );
}
