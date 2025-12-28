fn split_to_u8 (input: u64) -> [u8; 8] {
    let output = [
	input as u8,
	(input >> 8) as u8,
	(input >> 16) as u8,
	(input >> 24) as u8,
	(input >> 32) as u8,
	(input >> 40) as u8,
	(input >> 48) as u8,
	(input >> 56) as u8,
    ];
    return output;
}

fn vec_to_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|_v: Vec<T>| panic!("error converting vector to array"))
}

fn u8_chunk_to_u32_vec(chunk: &mut [u8]) -> Vec::<u32> {
    let mut x: Vec::<u32> = Vec::new();

    let mut temp_vec: Vec::<u8> = Vec::new();
    let mut count = 0;

    for i in 0..chunk.len() {
	temp_vec.push(chunk[i]);
	count += 1;
	if count == 4 {
	    let temp_arr: [u8; 4] = vec_to_array(temp_vec.clone());
	    let value = u32::from_ne_bytes(temp_arr);
	    x.push(value);
	    count = 0;
	    temp_vec.clear()
	}
    }

    return x;
}
#[allow(non_snake_case)]
fn func_F(x: u32, y: u32, z: u32) -> u32 {
    (x & y) | (!x & z)
}

#[allow(non_snake_case)]
fn func_G(x: u32, y: u32, z: u32) -> u32 {
    (x & z) | (y & !z)
}

#[allow(non_snake_case)]
fn func_H(x: u32, y: u32, z: u32) -> u32 {
    x ^ y ^ z
}

#[allow(non_snake_case)]
fn func_I(x: u32, y: u32, z: u32) -> u32 {
    y ^ (x | !z)
}
    
fn round_one(mut a: u32, mut b: u32, mut c: u32, mut d: u32, table: &Vec<u32>, chunk: &Vec<u32>) -> [u32; 4] {
    // a = b + ((a + f(b,c,d) + chunk[k] + table[i]) <<< s);
    macro_rules! round1 {
	( $a:ident, $b:ident, $c:ident, $d:ident, $k:expr, $s:expr, $i: expr ) => {
            $a = $b.wrapping_add(
                ($a.wrapping_add(func_F($b, $c, $d))
                    .wrapping_add(chunk[$k])
                    .wrapping_add(table[$i]))
                .rotate_left($s),
            )
        };
    }

    round1!(a, b, c, d, 0, 7, 1);
    round1!(d, a, b, c, 1, 12, 2);
    round1!(c, d, a, b, 2, 17, 3);
    round1!(b, c, d, a, 3, 22, 4);

    round1!(a, b, c, d, 4, 7, 5);
    round1!(d, a, b, c, 5, 12, 6);
    round1!(c, d, a, b, 6, 17, 7);
    round1!(b, c, d, a, 7, 22, 8);

    round1!(a, b, c, d, 8, 7, 9);
    round1!(d, a, b, c, 9, 12, 10);
    round1!(c, d, a, b, 10, 17, 11);
    round1!(b, c, d, a, 11, 22, 12);

    round1!(a, b, c, d, 12, 7, 13);
    round1!(d, a, b, c, 13, 12, 14);
    round1!(c, d, a, b, 14, 17, 15);
    round1!(b, c, d, a, 15, 22, 16);

    return [a, b, c, d];
}

fn round_two (mut a: u32, mut b: u32, mut c: u32, mut d: u32, table: &Vec<u32>, chunk: &Vec<u32>) -> [u32; 4] {
    // a = b + ((a + G(b,c,d) + X[k] + T[i]) <<< s)
    macro_rules! round_2 {
	( $a:ident, $b:ident, $c:ident, $d:ident, $k:expr, $s:expr, $i: expr ) => {
	    $a = $b.wrapping_add(
		($a.wrapping_add(func_G($b, $c, $d))
		 .wrapping_add(chunk[$k])
		 .wrapping_add(table[$i])
		 .rotate_left($s)
		)
	    )
	}
    }

    round_2! (a, b, c, d, 1, 5, 17);
    round_2! (d, a, b, c, 6, 9, 18);
    round_2! (c, d, a, b, 11, 14, 19);
    round_2! (b, c, d, a, 0, 20, 20);

    round_2! (a, b, c, d, 5, 5, 21);
    round_2! (d, a, b, c, 10, 9, 22);
    round_2! (c, d, a, b, 15, 14, 23);
    round_2! (b, c, d, a, 4, 20, 24);

    round_2! (a, b, c, d, 9, 5, 25);
    round_2! (d, a, b, c, 14, 9, 26);
    round_2! (c, d, a, b, 3, 14, 27);
    round_2! (b, c, d, a, 8, 20, 28);

    round_2! (a, b, c, d, 13, 5, 29);
    round_2! (d, a, b, c, 2, 9, 30);
    round_2! (c, d, a, b, 7, 14, 31);
    round_2! (b, c, d, a, 12, 20, 32);

    return [a, b, c, d];
}

fn round_three (mut a: u32, mut b: u32, mut c: u32, mut d: u32, table: &Vec<u32>, chunk: &Vec<u32>) -> [u32; 4] {
    //  a = b + ((a + H(b,c,d) + X[k] + T[i]) <<< s)
    macro_rules! round_3 {
	( $a:ident, $b:ident, $c:ident, $d:ident, $k:expr, $s:expr, $i: expr ) => {
	    $a = $b.wrapping_add(
		($a.wrapping_add(func_H($b, $c, $d))
		 .wrapping_add(chunk[$k]).
		 wrapping_add(table[$i]).
		 rotate_left($s)
		)
	    )
	}
    }
    
    round_3! (a, b, c, d, 5, 4, 33);
    round_3! (d, a, b, c, 8, 11, 34);
    round_3! (c, d, a, b, 11, 16, 35);
    round_3! (b, c, d, a, 14, 23, 36);

    round_3! (a, b, c, d, 1, 4, 37);
    round_3! (d, a, b, c, 4, 11, 38);
    round_3! (c, d, a, b, 7, 16, 39);
    round_3! (b, c, d, a, 10, 23, 40);

    round_3! (a, b, c, d, 13, 4, 41);
    round_3! (d, a, b, c, 0, 11, 42);
    round_3! (c, d, a, b, 3, 16, 43);
    round_3! (b, c, d, a, 6, 23, 44);

    round_3! (a, b, c, d, 9, 4, 45);
    round_3! (d, a, b, c, 12, 11, 46);
    round_3! (c, d, a, b, 15, 16, 47);
    round_3! (b, c, d, a, 2, 23, 48);

    return [a, b, c, d];
}

fn round_four (mut a: u32, mut b: u32, mut c: u32, mut d: u32, table: &Vec<u32>, chunk: &Vec<u32>) -> [u32; 4] {
    //  a = b + ((a + I(b,c,d) + X[k] + T[i]) <<< s)
    macro_rules! round_4 {
	( $a:ident, $b:ident, $c:ident, $d:ident, $k:expr, $s:expr, $i: expr ) => {
	    $a = $b.wrapping_add(
		($a.wrapping_add(func_I($b, $c, $d))
		 .wrapping_add(chunk[$k]).
		 wrapping_add(table[$i]).
		 rotate_left($s)
		)
	    )
	}
    }
    
    round_4! (a, b, c, d, 0, 6, 49);
    round_4! (d, a, b, c, 7, 10, 50);
    round_4! (c, d, a, b, 14, 15, 51);
    round_4! (b, c, d, a, 5, 21, 52);

    round_4! (a, b, c, d, 12, 6, 53);
    round_4! (d, a, b, c, 3, 10, 54);
    round_4! (c, d, a, b, 10, 15, 55);
    round_4! (b, c, d, a, 1, 21, 56);

    round_4! (a, b, c, d, 8, 6, 57);
    round_4! (d, a, b, c, 15, 10, 58);
    round_4! (c, d, a, b, 6, 15, 59);
    round_4! (b, c, d, a, 13, 21, 60);

    round_4! (a, b, c, d, 4, 6, 61);
    round_4! (d, a, b, c, 11, 10, 62);
    round_4! (c, d, a, b, 2, 15, 63);
    round_4! (b, c, d, a, 9, 21, 64);

    return [a, b, c, d];
}

fn md5hash<'a> (input: &str) -> String {
    //pad 1 to end, add enough zeros so the length is divisible by 512
    let original_length: u64 = (input.chars().count() as u64) * 8u64;
    let mut unhashed_vec: Vec::<u8> = Vec::new();
    unhashed_vec.extend(input.as_bytes());
    unhashed_vec.push(128u8);
    while (unhashed_vec.len() * 8) % 512 != 448 {
	unhashed_vec.push(0u8);
    }

    let converted_length = split_to_u8(original_length);
    unhashed_vec.extend(converted_length);

    let mut value_table: Vec::<u32> = Vec::new();
    value_table.push(0x00000000);
    for i in 1..=64 {
	value_table.push(((i as f64).sin().abs() * 4294967296.0) as u32);
    }
 
    let mut word_a = 0x67452301u32;
    let mut word_b = 0xefcdab89u32;
    let mut word_c = 0x98badcfeu32;
    let mut word_d = 0x10325476u32;

    for chunk in unhashed_vec.chunks_exact_mut(64) {
	let x = u8_chunk_to_u32_vec(chunk);

	let word_aa = word_a;
	let word_bb = word_b;
	let word_cc = word_c;
	let word_dd = word_d;

	let res = round_one(word_a, word_b, word_c, word_d, &value_table, &x);
	word_a = res[0];
	word_b = res[1];
	word_c = res[2];
	word_d = res[3];

	let res = round_two(word_a, word_b, word_c, word_d, &value_table, &x);
	word_a = res[0];
	word_b = res[1];
	word_c = res[2];
	word_d = res[3];
	
	let res = round_three(word_a, word_b, word_c, word_d, &value_table, &x);
	word_a = res[0];
	word_b = res[1];
	word_c = res[2];
	word_d = res[3];
	
	let res = round_four(word_a, word_b, word_c, word_d, &value_table, &x);
	word_a = res[0];
	word_b = res[1];
	word_c = res[2];
	word_d = res[3];

	word_a = word_a.wrapping_add(word_aa);
	word_b = word_b.wrapping_add(word_bb);
	word_c = word_c.wrapping_add(word_cc);
	word_d = word_d.wrapping_add(word_dd);
    }

    let digest = format!(
	"{:08x}{:08x}{:08x}{:08x}",
	word_a.swap_bytes(),
	word_b.swap_bytes(),
	word_c.swap_bytes(),
	word_d.swap_bytes(),
    );

    return digest;
}

fn main() {
    // Part one
    let input = "iwrupvqb";
    for index in 0..=1000000 {
	let inp = format!("{}{}", input, index);
	let output = md5hash(&inp);
	if output.starts_with("00000") {
	    println!("{}", index);
	    break;
	}
    }
    // Part two
    let input = "iwrupvqb";
    for index in 346387..=(1e8 as usize) {
	let inp = format!("{}{}", input, index);
	let output = md5hash(&inp);
	if output.starts_with("000000") {
	    println!("{}", index);
	    break;
	}
    }
}

