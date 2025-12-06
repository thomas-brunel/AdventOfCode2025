// From https://adventofcode.com/2025/day/3

// --- Day 3: Lobby ---

// You descend a short staircase, enter the surprisingly vast lobby, and are
// quickly cleared by the security checkpoint. When you get to the main
// elevators, however, you discover that each one has a red light above it:
// they're all offline.

// "Sorry about that," an Elf apologizes as she tinkers with a nearby control
// panel. "Some kind of electrical surge seems to have fried them. I'll try to
// get them online soon."

// You explain your need to get further underground. "Well, you could at least
// take the escalator down to the printing department, not that you'd get much
// further than that without the elevators working. That is, you could if the
// escalator weren't also offline."

// "But, don't worry! It's not fried; it just needs power. Maybe you can get it
// running while I keep working on the elevators."

// There are batteries nearby that can supply emergency power to the escalator
// for just such an occasion. The batteries are each labeled with their joltage
// rating, a value from 1 to 9. You make a note of their joltage ratings
// (your puzzle input). For example:

// 987654321111111
// 811111111111119
// 234234234234278
// 818181911112111

// The batteries are arranged into banks; each line of digits in your input
// corresponds to a single bank of batteries. Within each bank, you need to turn
// on exactly two batteries; the joltage that the bank produces is equal to the
// number formed by the digits on the batteries you've turned on. For example,
// if you have a bank like 12345 and you turn on batteries 2 and 4, the bank
// would produce 24 jolts. (You cannot rearrange batteries.)

// You'll need to find the largest possible joltage each bank can produce.
// In the above example:

//    In 987654321111111, you can make the largest joltage possible, 98,
//                        by turning on the first two batteries.
//    In 811111111111119, you can make the largest joltage possible by turning
//                        on the batteries labeled 8 and 9, producing 89 jolts.
//    In 234234234234278, you can make 78 by turning on the last two batteries
//                        (marked 7 and 8).
//    In 818181911112111, the largest joltage you can produce is 92.

// The total output joltage is the sum of the maximum joltage from each bank,
// so in this example, the total output joltage is 98 + 89 + 78 + 92 = 357.

// There are many batteries in front of you. Find the maximum joltage possible
// from each bank; what is the total output joltage?

// --- Part Two ---

// The escalator doesn't move. The Elf explains that it probably needs more
// joltage to overcome the static friction of the system and hits the big red
// "joltage limit safety override" button. You lose count of the number of times
// she needs to confirm "yes, I'm sure" and decorate the lobby a bit while you wait.

// Now, you need to make the largest joltage by turning on exactly twelve
// batteries within each bank.

// The joltage output for the bank is still the number formed by the digits of
// the batteries you've turned on; the only difference is that now there will be
// 12 digits in each bank's joltage output instead of two.

// Consider again the example from before:

// 987654321111111
// 811111111111119
// 234234234234278
// 818181911112111

// Now, the joltages are much larger:

//    In 987654321111111, the largest joltage can be found by turning on
//                        everything except some 1s at the end to produce 987654321111.
//    In the digit sequence 811111111111119, the largest joltage can be found by
//                                           turning on everything except some
//                                           1s, producing 811111111119.
//    In 234234234234278, the largest joltage can be found by turning on
//                        everything except a 2 battery, a 3 battery, and
//                        another 2 battery near the start to produce 434234234278.
//    In 818181911112111, the joltage 888911112111 is produced by turning on
//                        everything except some 1s near the front.

// The total output joltage is now much larger:
// 987654321111 + 811111111119 + 434234234278 + 888911112111 = 3121910778619.

// What is the new total output joltage?

use std::io::BufRead;

fn bank_output_joltage_part1(line: &String) -> u32 {
    let Some(first_battery) = line[..line.len() - 1]
        .chars()
        .filter_map(|c| c.to_digit(10))
        .max()
    else {
        panic!("Impossible to find a max digit!");
    };
    let Some(first_battery_position) = line.find(&first_battery.to_string()) else {
        panic!("Impossible to get the position of the first battery!");
    };
    let Some(second_battery) = line[(first_battery_position + 1)..]
        .chars()
        .filter_map(|c| c.to_digit(10))
        .max()
    else {
        panic!("Impossible to find a max digit!");
    };
    let max_joltage = first_battery * 10 + second_battery;
    println!("For bank {} - largest joltage found {}", line, max_joltage);
    max_joltage
}

fn bank_output_joltage_part2(line: &String, nb_max_battery: usize) -> u64 {
    let mut max_output_joltage: u64 = 0;
    let mut last_battery_position = 0;
    if line.len() < nb_max_battery {
        panic!("Bank size too small! ({} - {})", line, line.len());
    }
    for battery_index in 1..=nb_max_battery {
        let usable_line =
            &line[last_battery_position..(line.len() - (nb_max_battery - battery_index))];
        let Some(battery) = usable_line.chars().filter_map(|c| c.to_digit(10)).max() else {
            panic!("No battery found for index {}", battery_index);
        };
        let Some(battery_position) = usable_line.find(&battery.to_string()) else {
            panic!(
                "Position not found for last battery index {}",
                battery_index
            );
        };
        last_battery_position += battery_position + 1;
        if battery_index != nb_max_battery && last_battery_position >= line.len() {
            panic!(
                "Battery index {} already at the end of bank!",
                battery_index
            );
        }
        max_output_joltage = max_output_joltage * 10 + (battery as u64);
    }
    println!(
        "For bank {} - largest joltage found {}",
        line, max_output_joltage
    );
    return max_output_joltage;
}

fn main() {
    let mut p1_total_output_joltage = 0;
    let mut p2_total_output_joltage = 0;

    let Ok(file) = std::fs::File::open("input.txt") else {
        panic!("Failed to open file: `input.txt`");
    };

    let reader = std::io::BufReader::new(file);
    for line in reader.lines().map(|l| l.unwrap()) {
        p1_total_output_joltage += bank_output_joltage_part1(&line);
        p2_total_output_joltage += bank_output_joltage_part2(&line, 12);
    }
    println!(
        "[PART 1] Total output max joltage: {}",
        p1_total_output_joltage
    );
    println!(
        "[PART 2] Total output max joltage: {}",
        p2_total_output_joltage
    );
}
