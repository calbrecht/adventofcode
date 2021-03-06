extern crate aoc_io;

fn main() -> Result<(), ()> {
    let day: &str = "2";
    aoc_io::init_env();
    let client = aoc_io::build_client().unwrap();
    let input: String = aoc_io::fetch_input_text(&client, day).unwrap();

    println!("input: {:#?}", input);

    let data: Vec<(bool, bool)> = input.split_terminator("\n")
        .map(|x: &str| x.splitn(5, |c| c == '-' || c == ' ' || c == ':').collect())
        .map(|x: Vec<&str>|
             (x[0].parse::<usize>().unwrap(),
              x[1].parse::<usize>().unwrap(),
              x[4].matches(x[2]).collect::<Vec<&str>>().len(),
              x[2].chars().nth(0).unwrap(),
              x[4].chars().collect()))
        .map(|x: (usize, usize, usize, char, Vec<char>)|
             (x.0 <= x.2 && x.1 >= x.2,
              (x.3 == x.4[x.0 -1]) != (x.3 == x.4[x.1 -1])))
        .collect();

    println!("data: {:#?}", data);

    let lvl1: usize = data.iter()
        .filter(&|x: &&(bool, bool)| x.0)
        .collect::<Vec<&(bool, bool)>>()
        .len();

    let lvl2: usize = data.iter()
        .filter(&|x: &&(bool, bool)| x.1)
        .collect::<Vec<&(bool, bool)>>()
        .len();

    println!("level 1: {:#?}", lvl1);
    println!("level 2: {:#?}", lvl2);

    //println!("{:#?}", aoc_io::post_result_text(&client, day, "1", lvl1.to_string().as_str()));
    //println!("{:#?}", aoc_io::post_result_text(&client, day, "2", lvl2.to_string().as_str()));

    Ok(())
}
