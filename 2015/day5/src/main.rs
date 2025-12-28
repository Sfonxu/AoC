use std::fs;
use std::path::Path;

const VOWELS: [u8; 5] = [b'a', b'e', b'i', b'o', b'u'];
const NAUGHTY_LIST: [&[u8]; 4] = [b"ab", b"cd", b"pq", b"xy"];

fn check_string_part_one(s: &str) -> i32 {
    let mut double_letter = false;
    let mut three_wovels = false;
    let mut no_naughty_substring = true;
    let mut vowel_counter = 0;
    for c in s.as_bytes().windows(2) {
	if c[0] == c[1] {
	    double_letter = true;
	}
	for cc in VOWELS{
	    if c[0] == cc {
		vowel_counter += 1;
	    }
	}
	for cc in NAUGHTY_LIST {
	    if c == cc {
		no_naughty_substring = false;
	    }
	}
    }
    // as windows iterator misses the last wovel
    for c in VOWELS {
	if c == *s.as_bytes().last().unwrap() {
	    vowel_counter += 1;
	}
    }
    if vowel_counter >= 3 {
	three_wovels = true;
    }
    if double_letter && three_wovels & no_naughty_substring {
	return 1;
    }
    return 0;
}

fn check_string_part_two(s: &str) -> i32 {
    let mut pairs = false;
    let mut repeats = false;
    for c in s.as_bytes().windows(3) {
	if c[0] == c[2] {
	    repeats = true;
	}
    }

    let pairs_array: Vec::<&[u8]> = s.as_bytes().windows(2).collect();
    
    for (i, e) in pairs_array.iter().enumerate() {
	for (ii, ee) in pairs_array.iter().enumerate() { 
	    if (i as i32 - ii as i32).abs() > 1 && e == ee {
		pairs = true;
		break;
	    }
	}
    }

    if pairs && repeats {
	return 1;
    }
    return 0;
}

fn main() {
    // Part One and Two
    let path = Path::new("../input");
    let input = fs::read_to_string(path).unwrap();
    let mut nice_counter_part_one = 0;
    let mut nice_counter_part_two = 0;

    for line in input.lines() {
	nice_counter_part_one += check_string_part_one(line);
	nice_counter_part_two += check_string_part_two(line);
    }
    println!("There are {} nice strings on the list by the first set of rules", nice_counter_part_one);
    println!("There are {} nice strings on the list by the second set of rules", nice_counter_part_two);
}
