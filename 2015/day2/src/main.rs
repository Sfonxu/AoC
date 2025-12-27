use std::fs;
use std::path::Path;

fn main() {
    //part one
    let path = Path::new("input");
    let input = fs::read_to_string(path)
	.expect("Cannot read input");
    let mut sizes = Vec::<[i32; 3]>::new();
    let mut paper_needed = 0;
    for line in input.lines() {
	let mut temp_arr: [i32; 3] = [0; 3];
	let mut t_string: String = String::new();
	let mut j = 0;
	for (i,c) in line.char_indices() {
	    if c != 'x' {
		t_string.push(c);
	    }
	    if c == 'x' {
		temp_arr[j] = t_string.parse::<i32>().unwrap();
		t_string = String::new();
		j += 1;
	    }
	    if i == line.len()-1 {
		temp_arr[j] = t_string.parse::<i32>().unwrap();
	    }
	}
	sizes.push(temp_arr);
    }
    for size in &sizes {
	let mut areas = Vec::<i32>::new();
	let area1 = size[0]*size[1];
	let area2 = size[1]*size[2];
	let area3 = size[2]*size[0];
	areas.push(area1);
	areas.push(area2);
	areas.push(area3);
	let min_area = *areas
	    .iter()
	    .min()
	    .unwrap();
	paper_needed += 2*(area1+area2+area3);
	paper_needed += min_area;
    }
    println!("The elves need {} square feet of paper", paper_needed);

    //part two
    let mut ribbon_len = 0;
    for size in &sizes {
	let mut perimiters = Vec::<i32>::new();
	let perim1 = 2*size[0]+2*size[1];
	let perim2 = 2*size[1]+2*size[2];
	let perim3 = 2*size[0]+2*size[2];
	perimiters.push(perim1);
	perimiters.push(perim2);
	perimiters.push(perim3); 
	
	let vol = size[0]*size[1]*size[2];

	let min_peirm = *perimiters
	    .iter()
	    .min()
	    .unwrap();

	ribbon_len += min_peirm+vol;
    }
    println!("The elves need {} feet of ribbon", ribbon_len);
}
