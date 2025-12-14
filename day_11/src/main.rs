// From https://adventofcode.com/2025/day/11

// --- Day 11: Reactor ---

// You hear some loud beeping coming from a hatch in the floor of the factory,
// so you decide to check it out. Inside, you find several large electrical
// conduits and a ladder.

// Climbing down the ladder, you discover the source of the beeping: a large,
// toroidal reactor which powers the factory above. Some Elves here are
// hurriedly running between the reactor and a nearby server rack, apparently
// trying to fix something.

// One of the Elves notices you and rushes over. "It's a good thing you're here!
// We just installed a new server rack, but we aren't having any luck getting
// the reactor to communicate with it!" You glance around the room and see a
// tangle of cables and devices running from the server rack to the reactor.
// She rushes off, returning a moment later with a list of the devices and their
// outputs (your puzzle input).

// For example:

// aaa: you hhh
// you: bbb ccc
// bbb: ddd eee
// ccc: ddd eee fff
// ddd: ggg
// eee: out
// fff: out
// ggg: out
// hhh: ccc fff iii
// iii: out

// Each line gives the name of a device followed by a list of the devices to
// which its outputs are attached. So, bbb: ddd eee means that device bbb has
// two outputs, one leading to device ddd and the other leading to device eee.

// The Elves are pretty sure that the issue isn't due to any specific device,
// but rather that the issue is triggered by data following some specific path
// through the devices. Data only ever flows from a device through its outputs;
// it can't flow backwards.

// After dividing up the work, the Elves would like you to focus on the devices
// starting with the one next to you (an Elf hastily attaches a label which just
// says you) and ending with the main output to the reactor (which is the device
// with the label out).

// To help the Elves figure out which path is causing the issue, they need you
// to find every path from you to out.

// In this example, these are all of the paths from you to out:

//    Data could take the connection from you to bbb, then from bbb to ddd, then
//    from ddd to ggg, then from ggg to out.
//    Data could take the connection to bbb, then to eee, then to out.
//    Data could go to ccc, then ddd, then ggg, then out.
//    Data could go to ccc, then eee, then out.
//    Data could go to ccc, then fff, then out.

// In total, there are 5 different paths leading from you to out.

// How many different paths lead from you to out?

// --- Part Two ---

// Thanks in part to your analysis, the Elves have figured out a little bit
// about the issue. They now know that the problematic data path passes through
// both dac (a digital-to-analog converter) and fft (a device which performs a
// fast Fourier transform).

// They're still not sure which specific path is the problem, and so they now
// need you to find every path from svr (the server rack) to out. However, the
// paths you find must all also visit both dac and fft (in any order).

// For example:

// svr: aaa bbb
// aaa: fft
// fft: ccc
// bbb: tty
// tty: ccc
// ccc: ddd eee
// ddd: hub
// hub: fff
// eee: dac
// dac: fff
// fff: ggg hhh
// ggg: out
// hhh: out

// This new list of devices contains many paths from svr to out:

// svr,aaa,fft,ccc,ddd,hub,fff,ggg,out
// svr,aaa,fft,ccc,ddd,hub,fff,hhh,out
// svr,aaa,fft,ccc,eee,dac,fff,ggg,out
// svr,aaa,fft,ccc,eee,dac,fff,hhh,out
// svr,bbb,tty,ccc,ddd,hub,fff,ggg,out
// svr,bbb,tty,ccc,ddd,hub,fff,hhh,out
// svr,bbb,tty,ccc,eee,dac,fff,ggg,out
// svr,bbb,tty,ccc,eee,dac,fff,hhh,out

// However, only 2 paths from svr to out visit both dac and fft.

// Find all of the paths that lead from svr to out. How many of those paths
// visit both dac and fft?

use std::io::BufRead;

fn discover(
    key: &str,
    graph: &std::collections::HashMap<&str, Vec<&str>>,
    mut memo: &mut std::collections::HashMap<String, u32>,
) -> u32 {
    if key == "out" {
        return 1;
    } else if key == "you" {
        return 0;
    }

    match memo.get(key) {
        Some(count) => *count,
        None => {
            let mut count = 0;
            for v in &graph[key] {
                count += discover(v, &graph, &mut memo);
            }
            memo.insert(key.to_string(), count);
            count
        }
    }
}

fn part1_solver(lines: &[String]) {
    let mut graph: std::collections::HashMap<&str, Vec<&str>> = std::collections::HashMap::new();

    for line in lines {
        let Some((key, values)) = line.split_once(':') else {
            panic!("Wrong format!");
        };
        let v: Vec<&str> = values.split(' ').filter(|s| !s.is_empty()).collect();
        graph.insert(key, v);
    }

    if !graph.contains_key("you") {
        panic!("Key `you` not found!");
    }
    let mut memo: std::collections::HashMap<String, u32> = std::collections::HashMap::new();
    let mut nb_path = 0;
    for v in &graph["you"] {
        nb_path += discover(v, &graph, &mut memo);
    }

    println!("[PART 1] Number of path from `you` to `out`: {}", nb_path);
}

fn discover2(
    key: &str,
    graph: &std::collections::HashMap<&str, Vec<&str>>,
    memo: &mut std::collections::HashMap<String, u64>,
    has_dac: bool,
    has_fft: bool,
) -> u64 {
    if key == "out" {
        return if has_dac && has_fft { 1 } else { 0 };
    } else if key == "svr" {
        return 0;
    }

    let new_has_dac = has_dac || key == "dac";
    let new_has_fft = has_fft || key == "fft";
    let memo_key = format!("{}|{}|{}", key, new_has_dac, new_has_fft);

    if let Some(&count) = memo.get(&memo_key) {
        return count;
    }

    let mut count = 0;
    if let Some(neighbors) = graph.get(key) {
        for v in neighbors {
            count += discover2(v, graph, memo, new_has_dac, new_has_fft);
        }
    }

    memo.insert(memo_key, count);
    count
}

fn part2_solver(lines: &[String]) {
    let mut graph: std::collections::HashMap<&str, Vec<&str>> = std::collections::HashMap::new();

    for line in lines {
        let Some((key, values)) = line.split_once(':') else {
            panic!("Wrong format!");
        };
        let v: Vec<&str> = values.split(' ').filter(|s| !s.is_empty()).collect();
        graph.insert(key, v);
    }

    if !graph.contains_key("svr") {
        panic!("Key `svr` not found!");
    }

    let mut memo: std::collections::HashMap<String, u64> = std::collections::HashMap::new();
    let mut nb_path = 0;

    for v in &graph["svr"] {
        nb_path += discover2(v, &graph, &mut memo, false, false);
    }

    println!(
        "[PART 2] Number of paths from `svr` to `out` (through `dac` and `fft`): {}",
        nb_path
    );
}

fn main() {
    let Ok(file) = std::fs::File::open("input.txt") else {
        panic!("Failed to open file");
    };

    let reader = std::io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    part1_solver(&lines);

    let Ok(file) = std::fs::File::open("input.txt") else {
        panic!("Failed to open file");
    };

    let reader = std::io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    part2_solver(&lines);
}
