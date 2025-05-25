use std::fs;

fn main() {
    let file_path = "/home/sfonxu/AoC/2015/day1/input";
    let input = fs::read_to_string(file_path).expect("Cannot read file");
    //Part one
    let mut floor = 0;
    for c in input.chars() {
	if c == '(' {floor += 1;}
	if c == ')' {floor -= 1;}
    }
    println!("Santa is on floor {floor}");

    //Part two
    let mut floor = 0;
    let mut i = 0;
    for c in input.chars() {
	if floor < 0 {break;}
	if c == '(' {
	    floor += 1;
	    i += 1;
	}
	if c == ')' {
	    floor -= 1;
	    i += 1;
	}
    }
    println!("{floor}");
    println!("Santa goes to the basement on step {i}");
}
