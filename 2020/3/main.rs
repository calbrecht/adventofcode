extern crate aoc_io;

fn main() -> Result<(), ()> {
    let day: &str = "3";
    aoc_io::init_env();
    let client = aoc_io::build_client().unwrap();
    let input: String = aoc_io::fetch_input_text(&client, day).unwrap();
    //println!("input: {:#?}", input);

    let data: Vec<&str> = input.split_terminator("\n").collect();
    //println!("data: {:#?}", data);

    let range_x: Vec<usize> = (0..data[0].len()).collect();

    let lvl1: usize = range_x.iter().cycle().step_by(3).enumerate()
        .take_while(|x: &(usize, &usize)| x.0 < data.len())
        .map(|x: (usize, &usize)| data[x.0].chars().nth(*x.1))
        .filter(|x: &Option<char>| *x == Some('#'))
        .collect::<Vec<Option<char>>>()
        .len();

    println!("level 1: {:#?}", lvl1);

    //let lvl2: usize =
    //println!("level 2: {:#?}", lvl2);

    //println!("{:#?}", aoc_io::post_result_text(&client, day, "1", lvl1.to_string().as_str()));
    //println!("{:#?}", aoc_io::post_result_text(&client, day, "2", lvl2.to_string().as_str()));

    Ok(())
}
