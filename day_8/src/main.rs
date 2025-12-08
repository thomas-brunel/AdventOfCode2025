// From https://adventofcode.com/2025/day/8

// --- Day 8: Playground ---

// Equipped with a new understanding of teleporter maintenance, you confidently
// step onto the repaired teleporter pad.

// You rematerialize on an unfamiliar teleporter pad and find yourself in a vast
// underground space which contains a giant playground!

// Across the playground, a group of Elves are working on setting up an
// ambitious Christmas decoration project. Through careful rigging, they have
// suspended a large number of small electrical junction boxes.

// Their plan is to connect the junction boxes with long strings of lights. Most
// of the junction boxes don't provide electricity; however, when two junction
// boxes are connected by a string of lights, electricity can pass between those
// two junction boxes.

// The Elves are trying to figure out which junction boxes to connect so that
// electricity can reach every junction box. They even have a list of all of the
// junction boxes' positions in 3D space (your puzzle input).

// For example:

// 162,817,812
// 57,618,57
// 906,360,560
// 592,479,940
// 352,342,300
// 466,668,158
// 542,29,236
// 431,825,988
// 739,650,466
// 52,470,668
// 216,146,977
// 819,987,18
// 117,168,530
// 805,96,715
// 346,949,466
// 970,615,88
// 941,993,340
// 862,61,35
// 984,92,344
// 425,690,689

// This list describes the position of 20 junction boxes, one per line. Each
// position is given as X,Y,Z coordinates. So, the first junction box in the
// list is at X=162, Y=817, Z=812.

// To save on string lights, the Elves would like to focus on connecting pairs
// of junction boxes that are as close together as possible according to
// straight-line distance. In this example, the two junction boxes which are
// closest together are 162,817,812 and 425,690,689.

// By connecting these two junction boxes together, because electricity can flow
// between them, they become part of the same circuit. After connecting them,
// there is a single circuit which contains two junction boxes, and the
// remaining 18 junction boxes remain in their own individual circuits.

// Now, the two junction boxes which are closest together but aren't already
// directly connected are 162,817,812 and 431,825,988. After connecting them,
// since 162,817,812 is already connected to another junction box, there is now
// a single circuit which contains three junction boxes and an additional 17
// circuits which contain one junction box each.

// The next two junction boxes to connect are 906,360,560 and 805,96,715. After
// connecting them, there is a circuit containing 3 junction boxes, a circuit
// containing 2 junction boxes, and 15 circuits which contain one junction
// box each.

// The next two junction boxes are 431,825,988 and 425,690,689. Because these
// two junction boxes were already in the same circuit, nothing happens!

// This process continues for a while, and the Elves are concerned that they
// don't have enough extension cables for all these circuits. They would like to
// know how big the circuits will be.

// After making the ten shortest connections, there are 11 circuits: one circuit
// which contains 5 junction boxes, one circuit which contains 4 junction boxes,
// two circuits which contain 2 junction boxes each, and seven circuits which
// each contain a single junction box. Multiplying together the sizes of the
// three largest circuits (5, 4, and one of the circuits of size 2) produces 40.

// Your list contains many junction boxes; connect together the 1000 pairs of
// junction boxes which are closest together. Afterward, what do you get if you
// multiply together the sizes of the three largest circuits?

// --- Part Two ---

// The Elves were right; they definitely don't have enough extension cables.
// You'll need to keep connecting junction boxes together until they're all in
// one large circuit.

// Continuing the above example, the first connection which causes all of the
// junction boxes to form a single circuit is between the junction boxes at
// 216,146,977 and 117,168,530. The Elves need to know how far those junction
// boxes are from the wall so they can pick the right extension cable;
// multiplying the X coordinates of those two junction boxes (216 and 117)
// produces 25272.

// Continue connecting the closest unconnected pairs of junction boxes together
// until they're all in the same circuit. What do you get if you multiply
// together the X coordinates of the last two junction boxes you need to connect?

use std::{fmt::Display, io::BufRead};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct JunctionBoxPos {
    x: i64,
    y: i64,
    z: i64,
}

impl JunctionBoxPos {
    fn get_distance_squared(&self, other: &JunctionBoxPos) -> i64 {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)
    }
}

impl Display for JunctionBoxPos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        // Already in same circuit
        if root_x == root_y {
            return false;
        }

        if self.size[root_x] < self.size[root_y] {
            self.parent[root_x] = root_y;
            self.size[root_y] += self.size[root_x];
        } else {
            self.parent[root_y] = root_x;
            self.size[root_x] += self.size[root_y];
        }
        true
    }

    fn get_circuit_sizes(&mut self) -> Vec<usize> {
        let mut sizes: Vec<usize> = vec![];
        for i in 0..self.parent.len() {
            if self.find(i) == i {
                sizes.push(self.size[i]);
            }
        }
        sizes
    }
}

fn part1_solver(lines: &[String]) {
    let mut junction_boxes: Vec<JunctionBoxPos> = vec![];

    for line in lines {
        let coords: Vec<i64> = line
            .split(',')
            .map(|s| s.parse().expect("Invalid coordinate"))
            .collect();
        junction_boxes.push(JunctionBoxPos {
            x: coords[0],
            y: coords[1],
            z: coords[2],
        });
    }

    let n = junction_boxes.len();
    let mut all_pairs: Vec<(usize, usize, i64)> = vec![];
    for i in 0..n {
        for j in (i + 1)..n {
            let dist = junction_boxes[i].get_distance_squared(&junction_boxes[j]);
            all_pairs.push((i, j, dist));
        }
    }
    all_pairs.sort_by_key(|&(_, _, dist)| dist);

    let num_connections = if n == 20 { 10 } else { 1000 };
    let mut uf = UnionFind::new(n);
    let mut pairs_processed = 0;
    for (i, j, dist) in &all_pairs {
        pairs_processed += 1;
        if uf.union(*i, *j) {
            println!(
                "Connected {} and {} (distance: {:.2})",
                junction_boxes[*i],
                junction_boxes[*j],
                (*dist as f64).sqrt()
            );
        } else {
            println!(
                "Skipped {} and {} (already in same circuit)",
                junction_boxes[*i], junction_boxes[*j]
            );
        }

        if pairs_processed >= num_connections {
            break;
        }
    }
    println!("");

    let mut circuit_sizes = uf.get_circuit_sizes();
    circuit_sizes.sort_by(|a, b| b.cmp(a));

    println!(
        "There are {} circuits each of size: {:?}",
        circuit_sizes.len(),
        circuit_sizes
    );

    if circuit_sizes.len() >= 3 {
        let result = circuit_sizes[0] * circuit_sizes[1] * circuit_sizes[2];
        println!(
            "[PART 1] Product of three largest circuits: {} × {} × {} = {}",
            circuit_sizes[0], circuit_sizes[1], circuit_sizes[2], result
        );
    }
}

fn part2_solver(lines: &[String]) {
    let mut junction_boxes: Vec<JunctionBoxPos> = vec![];

    for line in lines {
        let coords: Vec<i64> = line
            .split(',')
            .map(|s| s.parse().expect("Invalid coordinate"))
            .collect();
        junction_boxes.push(JunctionBoxPos {
            x: coords[0],
            y: coords[1],
            z: coords[2],
        });
    }

    let n = junction_boxes.len();
    let mut all_pairs: Vec<(usize, usize, i64)> = vec![];
    for i in 0..n {
        for j in (i + 1)..n {
            let dist = junction_boxes[i].get_distance_squared(&junction_boxes[j]);
            all_pairs.push((i, j, dist));
        }
    }
    all_pairs.sort_by_key(|&(_, _, dist)| dist);

    let mut uf = UnionFind::new(n);
    let mut last_pair: (usize, usize) = (0, 0);
    let mut circuit_remaining = n;
    for (i, j, dist) in &all_pairs {
        if uf.union(*i, *j) {
            println!(
                "Connected {} and {} (distance: {:.2})",
                junction_boxes[*i],
                junction_boxes[*j],
                (*dist as f64).sqrt()
            );
            last_pair = (*i, *j);
            circuit_remaining -= 1;
        }

        if circuit_remaining == 1 {
            break;
        }
    }
    println!("");

    let result = junction_boxes[last_pair.0].x * junction_boxes[last_pair.1].x;
    println!(
        "[PART 2] Product of X coordinates of last connected junction boxes: {} × {} = {}",
        junction_boxes[last_pair.0].x, junction_boxes[last_pair.1].x, result
    );
}

fn main() {
    let Ok(file) = std::fs::File::open("input.txt") else {
        panic!("Failed to open file");
    };

    let reader = std::io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    part1_solver(&lines);
    part2_solver(&lines);
}
