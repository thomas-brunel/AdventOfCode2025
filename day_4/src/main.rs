// From https://adventofcode.com/2025/day/4

// --- Day 4: Printing Department ---

// You ride the escalator down to the printing department. They're clearly
// getting ready for Christmas; they have lots of large rolls of paper
// everywhere, and there's even a massive printer in the corner (to handle the
// really big print jobs).

// Decorating here will be easy: they can make their own decorations. What you
// really need is a way to get further into the North Pole base while the
// elevators are offline.

// "Actually, maybe we can help with that," one of the Elves replies when you
// ask for help. "We're pretty sure there's a cafeteria on the other side of the
// back wall. If we could break through the wall, you'd be able to keep moving.
// It's too bad all of our forklifts are so busy moving those big rolls of paper
// around."

// If you can optimize the work the forklifts are doing, maybe they would have
// time to spare to break through the wall.

// The rolls of paper (@) are arranged on a large grid; the Elves even have a
// helpful diagram (your puzzle input) indicating where everything is located.

// For example:

// ..@@.@@@@.
// @@@.@.@.@@
// @@@@@.@.@@
// @.@@@@..@.
// @@.@@@@.@@
// .@@@@@@@.@
// .@.@.@.@@@
// @.@@@.@@@@
// .@@@@@@@@.
// @.@.@@@.@.

// The forklifts can only access a roll of paper if there are fewer than four
// rolls of paper in the eight adjacent positions. If you can figure out which
// rolls of paper the forklifts can access, they'll spend less time looking and
// more time breaking down the wall to the cafeteria.

// In this example, there are 13 rolls of paper that can be accessed by a
// forklift (marked with x):

// ..xx.xx@x.
// x@@.@.@.@@
// @@@@@.x.@@
// @.@@@@..@.
// x@.@@@@.@x
// .@@@@@@@.@
// .@.@.@.@@@
// x.@@@.@@@@
// .@@@@@@@@.
// x.x.@@@.x.

// Consider your complete diagram of the paper roll locations. How many rolls of
// paper can be accessed by a forklift?

// --- Part Two ---

// Now, the Elves just need help accessing as much of the paper as they can.

// Once a roll of paper can be accessed by a forklift, it can be removed. Once a
// roll of paper is removed, the forklifts might be able to access more rolls of
// paper, which they might also be able to remove. How many total rolls of paper
// could the Elves remove if they keep repeating this process?

// Starting with the same example as above, here is one way you could remove as
// many rolls of paper as possible, using highlighted @ to indicate that a roll
// of paper is about to be removed, and using x to indicate that a roll of paper
// was just removed:

// Initial state:
// ..@@.@@@@.
// @@@.@.@.@@
// @@@@@.@.@@
// @.@@@@..@.
// @@.@@@@.@@
// .@@@@@@@.@
// .@.@.@.@@@
// @.@@@.@@@@
// .@@@@@@@@.
// @.@.@@@.@.

// Remove 13 rolls of paper:
// ..xx.xx@x.
// x@@.@.@.@@
// @@@@@.x.@@
// @.@@@@..@.
// x@.@@@@.@x
// .@@@@@@@.@
// .@.@.@.@@@
// x.@@@.@@@@
// .@@@@@@@@.
// x.x.@@@.x.

// Remove 12 rolls of paper:
// .......x..
// .@@.x.x.@x
// x@@@@...@@
// x.@@@@..x.
// .@.@@@@.x.
// .x@@@@@@.x
// .x.@.@.@@@
// ..@@@.@@@@
// .x@@@@@@@.
// ....@@@...

// Remove 7 rolls of paper:
// ..........
// .x@.....x.
// .@@@@...xx
// ..@@@@....
// .x.@@@@...
// ..@@@@@@..
// ...@.@.@@x
// ..@@@.@@@@
// ..x@@@@@@.
// ....@@@...

// Remove 5 rolls of paper:
// ..........
// ..x.......
// .x@@@.....
// ..@@@@....
// ...@@@@...
// ..x@@@@@..
// ...@.@.@@.
// ..x@@.@@@x
// ...@@@@@@.
// ....@@@...

// Remove 2 rolls of paper:
// ..........
// ..........
// ..x@@.....
// ..@@@@....
// ...@@@@...
// ...@@@@@..
// ...@.@.@@.
// ...@@.@@@.
// ...@@@@@x.
// ....@@@...

// Remove 1 roll of paper:
// ..........
// ..........
// ...@@.....
// ..x@@@....
// ...@@@@...
// ...@@@@@..
// ...@.@.@@.
// ...@@.@@@.
// ...@@@@@..
// ....@@@...

// Remove 1 roll of paper:
// ..........
// ..........
// ...x@.....
// ...@@@....
// ...@@@@...
// ...@@@@@..
// ...@.@.@@.
// ...@@.@@@.
// ...@@@@@..
// ....@@@...

// Remove 1 roll of paper:
// ..........
// ..........
// ....x.....
// ...@@@....
// ...@@@@...
// ...@@@@@..
// ...@.@.@@.
// ...@@.@@@.
// ...@@@@@..
// ....@@@...

// Remove 1 roll of paper:
// ..........
// ..........
// ..........
// ...x@@....
// ...@@@@...
// ...@@@@@..
// ...@.@.@@.
// ...@@.@@@.
// ...@@@@@..
// ....@@@...

// Stop once no more rolls of paper are accessible by a forklift. In this
// example, a total of 43 rolls of paper can be removed.

// Start with your original diagram. How many rolls of paper in total can be
// removed by the Elves and their forklifts?

use std::io::BufRead;

struct PaperRollPlaceHolder {
    status: bool,
    removable: bool,
}

impl PaperRollPlaceHolder {
    fn new(status: bool) -> Self {
        Self {
            status,
            removable: false,
        }
    }

    fn has_paper_roll(&self) -> bool {
        self.status
    }

    fn is_removable(&self) -> bool {
        self.removable
    }

    fn mark_removable(&mut self) {
        self.removable = true;
    }

    fn remove(&mut self) {
        self.status = false;
        self.removable = false;
    }
}

fn can_remove_paper_rolls(
    paper_rolls_grid: &mut Vec<Vec<PaperRollPlaceHolder>>,
    total_paper_rolls_removed: &mut u32,
) -> bool {
    let mut has_paper_roll_to_remove: bool = false;

    let directions = [
        (-1, -1), // top-left
        (-1, 0),  // top-mid
        (-1, 1),  // top-right
        (0, -1),  // mid-left
        (0, 1),   // mid-right
        (1, -1),  // bottom-left
        (1, 0),   // bottom-mid
        (1, 1),   // bottom-right
    ];

    let rows = paper_rolls_grid.len();

    for row in 0..rows {
        let cols = paper_rolls_grid[row].len();
        for col in 0..cols {
            if !paper_rolls_grid[row][col].has_paper_roll() {
                print!(".");
                continue;
            }

            let mut nb_adjacent_rolls = 0;

            for (dr, dc) in directions {
                let new_row = row as i32 + dr;
                let new_col = col as i32 + dc;

                // Check bounds
                if new_row >= 0 && new_row < rows as i32 && new_col >= 0 && new_col < cols as i32 {
                    nb_adjacent_rolls += paper_rolls_grid[new_row as usize][new_col as usize]
                        .has_paper_roll() as u32;
                }
            }

            if nb_adjacent_rolls < 4 {
                print!("x");
                paper_rolls_grid[row][col].mark_removable();
                *total_paper_rolls_removed += 1;
                has_paper_roll_to_remove = true;
            } else {
                print!("@");
            }
        }
        println!();
    }

    has_paper_roll_to_remove
}

fn remove_paper_rolls(paper_rolls_grid: &mut Vec<Vec<PaperRollPlaceHolder>>) {
    for paper_roll_line in paper_rolls_grid {
        for paper_roll in paper_roll_line {
            if paper_roll.is_removable() {
                paper_roll.remove();
            }
        }
    }
}

fn main() {
    let mut p1_total_paper_rolls_accessible: u32 = 0;
    let mut p2_total_paper_rolls_removed: u32 = 0;
    let mut paper_rolls_grid: Vec<Vec<PaperRollPlaceHolder>> = vec![];

    let Ok(file) = std::fs::File::open("input.txt") else {
        panic!("Failed to open file: `input.txt`");
    };

    let reader = std::io::BufReader::new(file);
    for line in reader.lines().map(|l| l.unwrap()) {
        let paper_rolls_line: Vec<PaperRollPlaceHolder> = line
            .chars()
            .map(|c| PaperRollPlaceHolder::new(c == '@'))
            .collect();
        paper_rolls_grid.push(paper_rolls_line);
    }

    let mut iteration = 1;
    println!("Iteration {}", iteration);
    while can_remove_paper_rolls(&mut paper_rolls_grid, &mut p2_total_paper_rolls_removed) {
        if iteration == 1 {
            p1_total_paper_rolls_accessible = p2_total_paper_rolls_removed;
        }
        remove_paper_rolls(&mut paper_rolls_grid);
        println!();
        iteration += 1;
        println!("Iteration {}", iteration);
    }

    println!(
        "[PART 1] Total accessible paper rolls by the forklift: {}",
        p1_total_paper_rolls_accessible
    );
    println!(
        "[PART 2] Total paper rolls removed by the forklift: {}",
        p2_total_paper_rolls_removed
    );
}
