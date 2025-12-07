// From https://adventofcode.com/2025/day/6

// --- Day 6: Trash Compactor ---

// After helping the Elves in the kitchen, you were taking a break and helping
// them re-enact a movie scene when you over-enthusiastically jumped into the
// garbage chute!

// A brief fall later, you find yourself in a garbage smasher. Unfortunately,
// the door's been magnetically sealed.

// As you try to find a way out, you are approached by a family of cephalopods!
// They're pretty sure they can get the door open, but it will take some time.
// While you wait, they're curious if you can help the youngest cephalopod with
// her math homework.

// Cephalopod math doesn't look that different from normal math. The math
// worksheet (your puzzle input) consists of a list of problems; each problem
// has a group of numbers that need to be either added (+) or multiplied (*)
// together.

// However, the problems are arranged a little strangely; they seem to be
// presented next to each other in a very long horizontal list.
// For example:

// 123 328  51 64
//  45 64  387 23
//   6 98  215 314
// *   +   *   +

// Each problem's numbers are arranged vertically; at the bottom of the problem
// is the symbol for the operation that needs to be performed. Problems are
// separated by a full column of only spaces. The left/right alignment of
// numbers within each problem can be ignored.

// So, this worksheet contains four problems:

//    123 * 45 * 6 = 33210
//    328 + 64 + 98 = 490
//    51 * 387 * 215 = 4243455
//    64 + 23 + 314 = 401

// To check their work, cephalopod students are given the grand total of adding
// together all of the answers to the individual problems. In this worksheet,
// the grand total is 33210 + 490 + 4243455 + 401 = 4277556.

// Of course, the actual worksheet is much wider. You'll need to make sure to
// unroll it completely so that you can read the problems clearly.

// Solve the problems on the math worksheet. What is the grand total found by
// adding together all of the answers to the individual problems?

// --- Part Two ---

// The big cephalopods come back to check on how things are going. When they see
// that your grand total doesn't match the one expected by the worksheet, they
// realize they forgot to explain how to read cephalopod math.

// Cephalopod math is written right-to-left in columns. Each number is given in
// its own column, with the most significant digit at the top and the least
// significant digit at the bottom. (Problems are still separated with a column
// consisting only of spaces, and the symbol at the bottom of the problem is
// still the operator to use.)

// Here's the example worksheet again:

// 123 328  51 64
//  45 64  387 23
//   6 98  215 314
// *   +   *   +

// Reading the problems right-to-left one column at a time, the problems are now
// quite different:

//    The rightmost problem is 4 + 431 + 623 = 1058
//    The second problem from the right is 175 * 581 * 32 = 3253600
//    The third problem from the right is 8 + 248 + 369 = 625
//    Finally, the leftmost problem is 356 * 24 * 1 = 8544

// Now, the grand total is 1058 + 3253600 + 625 + 8544 = 3263827.

// Solve the problems on the math worksheet again. What is the grand total found
// by adding together all of the answers to the individual problems?

use std::io::BufRead;

fn part1_solver(lines: &[String]) {
    let grid_problems: Vec<Vec<String>> = lines
        .iter()
        .map(|line| line.split_whitespace().map(|s| s.to_string()).collect())
        .collect();
    let rows = grid_problems.len();
    let mut res_problems: Vec<u64> = grid_problems[0]
        .iter()
        .map(|s| {
            s.parse::<u64>()
                .expect(&format!("Failed to convert first row {s}!"))
        })
        .collect();
    for row in 1..(rows - 1) {
        let cols = grid_problems[row].len();
        for col in 0..cols {
            let operator = &grid_problems[rows - 1][col];
            let value = &grid_problems[row][col];
            let Ok(val) = value.parse::<u64>() else {
                panic!("Failed to convert value {value} at row: {row}, col:{col}");
            };
            match operator.as_str() {
                "+" => res_problems[col] += val,
                "*" => res_problems[col] *= val,
                _ => panic!("Unknown operator {}", operator),
            };
        }
    }

    let p1_total_of_individual_problems: u64 = res_problems.iter().sum();

    println!("[PART 1] Grand total of individual problems {p1_total_of_individual_problems}");
}

fn part2_solver(lines: &[String]) {
    let rows = lines.len();
    let Some(max_col) = lines.iter().map(|l| l.len()).max() else {
        panic!("Failed to find max column number of all problems!");
    };

    let mut problems: Vec<Vec<u64>> = vec![];
    let mut operators: Vec<String> = vec![];
    let mut current_problem: Vec<u64> = vec![];

    for col in (0..max_col).rev() {
        let mut column_chars: Vec<char> = vec![];
        let mut all_spaces = true;

        for row in 0..rows {
            let c = lines[row].chars().nth(col).unwrap_or(' ');
            column_chars.push(c);
            if c != ' ' {
                all_spaces = false;
            }
        }

        if all_spaces {
            if !current_problem.is_empty() {
                problems.push(current_problem.clone());
                current_problem.clear();
            }
            continue;
        }

        let mut number = String::new();
        for row in 0..(rows - 1) {
            let c = column_chars[row];
            if c != ' ' {
                number.push(c);
            }
        }

        if !number.is_empty() {
            if let Ok(n) = number.parse::<u64>() {
                current_problem.push(n);
            }
        }
        let operator = column_chars[rows - 1].to_string();
        if operator == "+" || operator == "*" {
            operators.push(operator);
        }
    }

    if !current_problem.is_empty() {
        problems.push(current_problem);
    }

    let mut grand_total: u64 = 0;
    for (idx, problem) in problems.iter().enumerate() {
        if problem.is_empty() || idx >= operators.len() {
            continue;
        }

        let operator = &operators[idx];
        let mut result = 1;

        match operator.as_str() {
            "+" => result = problem.iter().sum(),
            "*" => {
                for num in problem {
                    result *= num;
                }
            }
            _ => panic!("Problem {idx}: Unknown operator {operator}"),
        }

        println!("Problem {}: {}", idx + 1, result);
        grand_total += result;
    }

    println!("[PART 2] Grand total: {}", grand_total);
}

fn main() {
    let Ok(file) = std::fs::File::open("input.txt") else {
        panic!("Failed to open file: `input.txt`");
    };

    let reader = std::io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    part1_solver(&lines);
    part2_solver(&lines);
}
