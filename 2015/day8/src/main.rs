use std::path::Path;
use std::fs;

fn count_chars_per_line(line: &str) -> usize {
    let chars: Vec::<char> = line.chars().collect();
    return chars.len();
}

fn count_chars_per_line_escaped(line: &str) -> usize {
    let chars: Vec::<char> = line.chars().collect();
    let mut count = 0;
    let mut i = 0;
    while i < chars.len() {
	if chars[i] == '\\' {
	    if chars[i+1] == '\\' || chars[i+1] == '\"' {
		i += 2;
		count += 1;
	    }
	    else if chars[i+1] == 'x' {
		i += 4;
		count += 1;
	    }
	}
	else {
	    i += 1;
	    count += 1;
	}
    }
    return count-2;
}

fn count_encode_line(line: &str) -> usize {
    let chars: Vec::<char> = line.chars().collect();
    let mut count = chars.len();
    for c in chars {
	if c == '\\' || c == '\"' {
	    count += 1; 
	}
    }
    return count+2;
}

fn main() {
    let path = Path::new("../input");
    let input = fs::read_to_string(path).expect("Failed to read input!");
    // Note - input.chars().count() would count newline characters!
    let mut full_count = 0;
    let mut string_count = 0;
    let mut encoded_count = 0;
    for line in input.lines() {
	full_count += count_chars_per_line(line);
	string_count += count_chars_per_line_escaped(line);
	encoded_count += count_encode_line(line);
    }
    println!("Part one character count is {}", full_count - string_count);
    println!("Part two charatcter count is {}", encoded_count-full_count);
}
