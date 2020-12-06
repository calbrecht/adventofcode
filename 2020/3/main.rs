extern crate aoc_io;

fn traverse (data: &Vec<&str>, step_x: usize, step_y: usize) -> usize {
    let range_x: Vec<usize> = (0..data[0].len()).collect();

    range_x.iter().cycle().step_by(step_x).enumerate()
        .take_while(|x: &(usize, &usize)| x.0 * step_y < data.len())
        .map(|x: (usize, &usize)| data[x.0 * step_y].chars().nth(*x.1))
        .filter(|x: &Option<char>| *x == Some('#'))
        .collect::<Vec<Option<char>>>()
        .len()
}

fn main() -> Result<(), ()> {
    let day: &str = "3";
    aoc_io::init_env();
    let client = aoc_io::build_client().unwrap();
    let input: String = aoc_io::fetch_input_text(&client, day).unwrap();
    //println!("input: {:#?}", input);

    let data: Vec<&str> = input.split_terminator("\n").collect();
    println!("data: {:#?}", data);

    let lvl1: usize = traverse(&data, 3, 1);

    let lvl2: usize =
        traverse(&data, 1, 1)
        * lvl1
        * traverse(&data, 5, 1)
        * traverse(&data, 7, 1)
        * traverse(&data, 1, 2);

    println!("level 1: {:#?}", lvl1);
    println!("level 2: {:#?}", lvl2);

    //println!("{:#?}", aoc_io::post_result_text(&client, day, "1", lvl1.to_string().as_str()));
    //println!("{:#?}", aoc_io::post_result_text(&client, day, "2", lvl2.to_string().as_str()));

    Ok(())
}
