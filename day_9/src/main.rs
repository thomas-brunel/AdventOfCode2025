// From https://adventofcode.com/2025/day/9

// --- Day 9: Movie Theater ---

// You slide down the firepole in the corner of the playground and land in the
// North Pole base movie theater!

// The movie theater has a big tile floor with an interesting pattern. Elves
// here are redecorating the theater by switching out some of the square tiles
// in the big grid they form. Some of the tiles are red; the Elves would like to
// find the largest rectangle that uses red tiles for two of its opposite
// corners. They even have a list of where the red tiles are located in the grid
// (your puzzle input).

// For example:

// 7,1
// 11,1
// 11,7
// 9,7
// 9,5
// 2,5
// 2,3
// 7,3

// Showing red tiles as # and other tiles as ., the above arrangement of red
// tiles would look like this:

// ..............
// .......#...#..
// ..............
// ..#....#......
// ..............
// ..#......#....
// ..............
// .........#.#..
// ..............

// You can choose any two red tiles as the opposite corners of your rectangle;
// your goal is to find the largest rectangle possible.

// For example, you could make a rectangle (shown as O) with an area of 24
// between 2,5 and 9,7:

// ..............
// .......#...#..
// ..............
// ..#....#......
// ..............
// ..OOOOOOOO....
// ..OOOOOOOO....
// ..OOOOOOOO.#..
// ..............

// Or, you could make a rectangle with area 35 between 7,1 and 11,7:

// ..............
// .......OOOOO..
// .......OOOOO..
// ..#....OOOOO..
// .......OOOOO..
// ..#....OOOOO..
// .......OOOOO..
// .......OOOOO..
// ..............

// You could even make a thin rectangle with an area of only 6 between 7,3 and 2,3:

// ..............
// .......#...#..
// ..............
// ..OOOOOO......
// ..............
// ..#......#....
// ..............
// .........#.#..
// ..............

// Ultimately, the largest rectangle you can make in this example has area 50.
// One way to do this is between 2,5 and 11,1:

// ..............
// ..OOOOOOOOOO..
// ..OOOOOOOOOO..
// ..OOOOOOOOOO..
// ..OOOOOOOOOO..
// ..OOOOOOOOOO..
// ..............
// .........#.#..
// ..............

// Using two red tiles as opposite corners, what is the largest area of any
// rectangle you can make?

// --- Part Two ---

// The Elves just remembered: they can only switch out tiles that are red or
// green. So, your rectangle can only include red or green tiles.

// In your list, every red tile is connected to the red tile before and after it
// by a straight line of green tiles. The list wraps, so the first red tile is
// also connected to the last red tile. Tiles that are adjacent in your list
// will always be on either the same row or the same column.

// Using the same example as before, the tiles marked X would be green:

// ..............
// .......#XXX#..
// .......X...X..
// ..#XXXX#...X..
// ..X........X..
// ..#XXXXXX#.X..
// .........X.X..
// .........#X#..
// ..............

// In addition, all of the tiles inside this loop of red and green tiles are
// also green. So, in this example, these are the green tiles:

// ..............
// .......#XXX#..
// .......XXXXX..
// ..#XXXX#XXXX..
// ..XXXXXXXXXX..
// ..#XXXXXX#XX..
// .........XXX..
// .........#X#..
// ..............

// The remaining tiles are never red nor green.

// The rectangle you choose still must have red tiles in opposite corners, but
// any other tiles it includes must now be red or green. This significantly
// limits your options.

// For example, you could make a rectangle out of red and green tiles with an
// area of 15 between 7,3 and 11,1:

// ..............
// .......OOOOO..
// .......OOOOO..
// ..#XXXXOOOOO..
// ..XXXXXXXXXX..
// ..#XXXXXX#XX..
// .........XXX..
// .........#X#..
// ..............

// Or, you could make a thin rectangle with an area of 3 between 9,7 and 9,5:

// ..............
// .......#XXX#..
// .......XXXXX..
// ..#XXXX#XXXX..
// ..XXXXXXXXXX..
// ..#XXXXXXOXX..
// .........OXX..
// .........OX#..
// ..............

// The largest rectangle you can make in this example using only red and green
// tiles has area 24. One way to do this is between 9,5 and 2,3:

// ..............
// .......#XXX#..
// .......XXXXX..
// ..OOOOOOOOXX..
// ..OOOOOOOOXX..
// ..OOOOOOOOXX..
// .........XXX..
// .........#X#..
// ..............

// Using two red tiles as opposite corners, what is the largest area of any
// rectangle you can make using only red and green tiles?

use std::io::BufRead;

fn part1_solver(lines: &[String]) {
    let red_tiles: Vec<(i64, i64)> = lines
        .iter()
        .map(|s| {
            let Some((coord_x, coord_y)) = s.split_once(',') else {
                panic!("Wrong format for line: {}", s);
            };
            let x = coord_x.parse().expect("Invalid coordinate x.");
            let y = coord_y.parse().expect("Invalid coordinate y.");
            (x, y)
        })
        .collect();

    let nb_red_tiles = red_tiles.len();
    println!("Got {} red tiles on the floor", nb_red_tiles);

    let mut largest_area = 0;
    for i in 0..nb_red_tiles {
        for j in i + 1..nb_red_tiles {
            let (x1, y1) = red_tiles[i];
            let (x2, y2) = red_tiles[j];
            let area = (x1.abs_diff(x2) + 1) * (y1.abs_diff(y2) + 1);
            if area > largest_area {
                largest_area = area;
            }
        }
    }

    println!(
        "[PART 1] The largest area of any rectangle is {}",
        largest_area
    );
}

fn point_in_polygon(point: (i64, i64), polygon: &[(i64, i64)]) -> bool {
    let (x, y) = point;
    let n = polygon.len();
    let mut inside = false;

    let mut j = n - 1;
    for i in 0..n {
        let (xi, yi) = polygon[i];
        let (xj, yj) = polygon[j];

        if ((yi > y) != (yj > y)) && (x < (xj - xi) * (y - yi) / (yj - yi) + xi) {
            inside = !inside;
        }
        j = i;
    }
    inside
}

fn on_polygon_edge(point: (i64, i64), edges: &[((i64, i64), (i64, i64))]) -> bool {
    let (x, y) = point;
    for &((x1, y1), (x2, y2)) in edges {
        if x1 == x2 {
            if x == x1 && y >= y1.min(y2) && y <= y1.max(y2) {
                return true;
            }
        } else if y1 == y2 {
            if y == y1 && x >= x1.min(x2) && x <= x1.max(x2) {
                return true;
            }
        }
    }
    false
}

fn is_area_valid(
    x1: i64,
    y1: i64,
    x2: i64,
    y2: i64,
    red_tiles: &[(i64, i64)],
    edges: &[((i64, i64), (i64, i64))],
) -> bool {
    let min_x = x1.min(x2);
    let max_x = x1.max(x2);
    let min_y = y1.min(y2);
    let max_y = y1.max(y2);

    let corners = [
        (min_x, min_y),
        (min_x, max_y),
        (max_x, min_y),
        (max_x, max_y),
    ];

    for &corner in &corners {
        if !on_polygon_edge(corner, edges) && !point_in_polygon(corner, red_tiles) {
            return false;
        }
    }

    let edge_samples = 100;

    for i in 1..edge_samples {
        let x = min_x + ((max_x - min_x) * i) / edge_samples;

        for &y in &[min_y, max_y] {
            if !on_polygon_edge((x, y), edges) && !point_in_polygon((x, y), red_tiles) {
                return false;
            }
        }
    }

    for i in 1..edge_samples {
        let y = min_y + ((max_y - min_y) * i) / edge_samples;

        for &x in &[min_x, max_x] {
            if !on_polygon_edge((x, y), edges) && !point_in_polygon((x, y), red_tiles) {
                return false;
            }
        }
    }

    let interior_samples = 10;
    for i in 1..interior_samples {
        let x = min_x + ((max_x - min_x) * i) / interior_samples;
        let y = min_y + ((max_y - min_y) * i) / interior_samples;

        if !on_polygon_edge((x, y), edges) && !point_in_polygon((x, y), red_tiles) {
            return false;
        }
    }

    true
}

fn part2_solver(lines: &[String]) {
    let red_tiles: Vec<(i64, i64)> = lines
        .iter()
        .map(|s| {
            let Some((coord_x, coord_y)) = s.split_once(',') else {
                panic!("Wrong format for line: {}", s);
            };
            let x = coord_x.parse().expect("Invalid coordinate x.");
            let y = coord_y.parse().expect("Invalid coordinate y.");
            (x, y)
        })
        .collect();

    let nb_red_tiles = red_tiles.len();

    let mut edges: Vec<((i64, i64), (i64, i64))> = Vec::new();
    for i in 0..nb_red_tiles {
        let p1 = red_tiles[i];
        let p2 = red_tiles[(i + 1) % nb_red_tiles];
        edges.push((p1, p2));
    }

    let mut largest_area: i64 = 0;
    for i in 0..nb_red_tiles {
        for j in i + 1..nb_red_tiles {
            let (x1, y1) = red_tiles[i];
            let (x2, y2) = red_tiles[j];

            if is_area_valid(x1, y1, x2, y2, &red_tiles, &edges) {
                let area = ((x1.abs_diff(x2) + 1) * (y1.abs_diff(y2) + 1)) as i64;
                if area > largest_area {
                    largest_area = area;
                }
            }
        }
    }

    println!(
        "[PART 2] The largest area of any rectangle using only red and green tiles is {}",
        largest_area
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
