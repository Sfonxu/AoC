use std::path::Path;
use std::fs;

// for printing
#[derive(Debug, PartialEq, Copy, Clone)]
enum Kind {
    Corner = 0,
    ActionOn,
    ActionOff,
    ActionToggle,
}

struct Command {
    corner1: (usize, usize),
    corner2: (usize, usize),
    action: Kind
}

struct Token {
    data: String,
    kind: Kind,
}

fn tokenizer(s: &str) -> Vec::<Token> {
    let mut temp_data = String::new();
    let mut tokens: Vec::<Token> = Vec::new();
    for c in s.chars() {
	if c != ' ' {
	    temp_data.push(c);
	} else {
	    if temp_data == "turn" || temp_data == "through" {
		temp_data.clear();
	    }
	    if temp_data == "on" || temp_data == "off" || temp_data == "toggle" {
		let action = match &*temp_data {
		    "toggle" => Kind::ActionToggle,
		    "on" => Kind::ActionOn,
		    "off" => Kind::ActionOff,
		    _ => {panic!("Undefined action kind"); }
		};
		tokens.push(Token{data: temp_data.clone(), kind: action});
		temp_data.clear();
	    }
	    if temp_data != "" {
		tokens.push(Token{data: temp_data.clone(), kind: Kind::Corner});
		temp_data.clear();
	    }
	    else {
		continue;
	    }
	}
    }
    tokens.push(Token{data: temp_data.clone(), kind: Kind::Corner});
    return tokens;
}

fn corner_from_string(s: String) -> (usize, usize) {
    let nums: Vec::<&str> = s.split(',').collect();
    let mut retval: (usize, usize) = (0, 0);
    retval.0 = nums[0].parse::<usize>().unwrap();
    retval.1 = nums[1].parse::<usize>().unwrap();
    return retval;
}

fn construct_command(line: &str) -> Command {
    let tokens = tokenizer(line);
    return Command{
	action: tokens[0].kind,
	corner1: corner_from_string(tokens[1].data.clone()),
	corner2: corner_from_string(tokens[2].data.clone()),
    };
}

fn command_on_part_one(mut lights: Vec::<[bool; 1000]>, command: &Command) -> Vec::<[bool; 1000]> {
    for i in (command.corner1.0)..=(command.corner2.0) {
	for j in (command.corner1.1)..=(command.corner2.1) {
	    lights[i][j] = true;
	}
    }
    return lights;
}

fn command_off_part_one(mut lights: Vec::<[bool; 1000]>, command: &Command) -> Vec::<[bool; 1000]> {
    for i in (command.corner1.0)..=(command.corner2.0) {
	for j in (command.corner1.1)..=(command.corner2.1) {
	    lights[i][j] = false;
	}
    }
    return lights;
}

fn command_toggle_part_one(mut lights: Vec::<[bool; 1000]>, command: &Command) -> Vec<[bool; 1000]> {
    for i in (command.corner1.0)..=(command.corner2.0) {
	for j in (command.corner1.1)..=(command.corner2.1) {
	    lights[i][j] = !lights[i][j];
	}
    }
    return lights;
}

fn command_on_part_two(mut lights: Vec::<[i32; 1000]>, command: &Command) -> Vec::<[i32; 1000]> {
    for i in (command.corner1.0)..=(command.corner2.0) {
	for j in (command.corner1.1)..=(command.corner2.1) {
	    lights[i][j] += 1;
	}
    }
    return lights;
}

fn command_off_part_two(mut lights: Vec::<[i32; 1000]>, command: &Command) -> Vec::<[i32; 1000]> {
    for i in (command.corner1.0)..=(command.corner2.0) {
	for j in (command.corner1.1)..=(command.corner2.1) {
	    if lights[i][j] > 0 {
		lights[i][j] -= 1;
	    } else {}
	}
    }
    return lights;
}

fn command_toggle_part_two(mut lights: Vec::<[i32; 1000]>, command: &Command) -> Vec<[i32; 1000]> {
    for i in (command.corner1.0)..=(command.corner2.0) {
	for j in (command.corner1.1)..=(command.corner2.1) {
	    lights[i][j] += 2;
	}
    }
    return lights;
}




fn check_lights_part_one(lights: &Vec::<[bool; 1000]>) -> i32 {
    let mut counter = 0i32;
    for arr in lights {
	for i in 0..1000 {
	    if arr[i] == true {
		counter += 1;
	    }
	}
    }
    return counter;
}

fn check_lights_part_two(lights: &Vec::<[i32; 1000]>) -> i32 {
    let mut counter = 0i32;
    for arr in lights {
	for i in 0..1000 {
	    counter += arr[i];
	}
    }
    return counter;
}

fn main() {
    let path = Path::new("../input");
    let mut lights_one: Vec::<[bool; 1000]> = Vec::new();
    let mut lights_two: Vec::<[i32; 1000]> = Vec::new();
    for _ in 0..1000 {
	lights_one.push([false; 1000]);
	lights_two.push([0; 1000]);
    }
    if let Ok(input) = fs::read_to_string(path) {
       	for line in input.lines() {
     	    let command = construct_command(line);
	    match command.action {
		Kind::ActionOn => {
		    lights_one = command_on_part_one(lights_one, &command);
		    lights_two = command_on_part_two(lights_two, &command);
		}
		Kind::ActionOff => {
		    lights_one = command_off_part_one(lights_one, &command);
		    lights_two = command_off_part_two(lights_two, &command);
		}
		Kind::ActionToggle => {
		    lights_one = command_toggle_part_one(lights_one, &command);
		    lights_two = command_toggle_part_two(lights_two, &command);
		}
		_ => {}
	    }
    	}
	let count_one = check_lights_part_one(&lights_one);
	let count_two = check_lights_part_two(&lights_two);
	println!("There are {} ligths on", count_one);
	println!("The intensity is {}", count_two);
    }
}

