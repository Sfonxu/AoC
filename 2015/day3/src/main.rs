use std::fs;
use std::collections::HashMap;

fn send_presents(map: &mut HashMap::<(i32, i32), i32>, c: char, position: &mut (i32, i32)) {
    match c {
	    '^' => {
		position.1 += 1;
		map.entry(*position)
		    .and_modify(|presents| *presents+=1)
		    .or_insert(1);
	    }
	    'v' => {
		position.1 -= 1;
		map.entry(*position)
		    .and_modify(|presents| *presents+=1)
		    .or_insert(1);
	    }
	    '<' => {
		position.0 -= 1;
		map.entry(*position)
		    .and_modify(|presents| *presents+=1)
		    .or_insert(1);
	    }
	    '>' => {
		position.0 += 1;
		map.entry(*position)
		    .and_modify(|presents| *presents+=1)
		    .or_insert(1);
	    }
	    _ => {}
    }
}

fn main() {
    let path = "/home/sfonxu/AoC/2015/day3/input";
    let input = fs::read_to_string(path).unwrap();
    let mut points: HashMap::<(i32,i32), i32> = HashMap::new();
    let mut position = (0,0);
    //part one 
    for c in input.chars() {
	send_presents(&mut points, c, &mut position);
    }
    let delivered = points.keys().len()-1+1; //one present at starting loc
    println!("There were {} houses with presents delivered", delivered);

    //part two
    let mut points_two: HashMap::<(i32,i32), i32> = HashMap::new();
    let mut robo_pos = (0,0);
    let mut real_pos = (0,0);
    for (i, c) in input.char_indices() {
	if i%2 == 0 {
	    send_presents(&mut points_two, c, &mut real_pos); 
	}
	if i%2 == 1 {
	    send_presents(&mut points_two, c, &mut robo_pos); 
	}
    }
    let delivered_two = points_two.keys().len(); //two presents get delivered at the start
    println!("With robo santa in action, {} hosues got a present", delivered_two);
}
