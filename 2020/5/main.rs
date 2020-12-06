extern crate aoc_io;

fn seat(b: (i8, i8), s: &str) -> (i8, i8) {
    //println!("u: {:#?} {:#?}", b, s);

    match b.0 == b.1 && s.len() > 0 {
        true => (b.0, seat((0, 7), &s[..]).1),
        false => match &s.chars().nth(0) {
            Some('B')|Some('R') => seat((b.1 - (b.1 - b.0)/2, b.1), &s[1..]),
            Some('F')|Some('L') => seat((b.0, b.0 + (b.1 - b.0)/2), &s[1..]),
            _ => b,
        }
    }
}

fn main() -> Result<(), ()> {
    let day: &str = "5";
    aoc_io::init_env();
    let client = aoc_io::build_client().unwrap();
    let input: String = aoc_io::fetch_input_text(&client, day).unwrap();
    //println!("input: {:#?}", input);

    let data: Vec<&str> = input.split_terminator("\n").collect();
    //println!("data: {:#?}", data);

    let mut lvl1 = data.iter()
        .map(|s: &&str| seat((0, 127), s))
        .map(|x: (i8, i8)| x.0 as i16 * 8 + x.1 as i16)
        .collect::<Vec<i16>>();

    lvl1.sort();

    let lvl2: i16 = lvl1.iter()
        .zip(*lvl1.first().unwrap()..=*lvl1.last().unwrap())
        .filter(|(a,b)| *a == b)
        .last()
        .unwrap()
        .0 + 1;

    println!("level 1: {:#?}", lvl1.last().unwrap());
    println!("level 2: {:#?}", lvl2);

    //println!("{:#?}", aoc_io::post_result_text(&client, day, "1", lvl1.to_string().as_str()));
    //println!("{:#?}", aoc_io::post_result_text(&client, day, "2", lvl2.to_string().as_str()));

    Ok(())
}

//BFFFBBFRRR: row 70, column 7, seat ID 567.
//FFFBBBFRRR: row 14, column 7, seat ID 119.
//BBFFBBFRLL: row 102, column 4, seat ID 820.
