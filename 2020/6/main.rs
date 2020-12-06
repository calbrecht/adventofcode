extern crate aoc_io;

fn count_answers_lvl1(v: Vec<&str>) -> usize {
    v.iter()
        .map(|s| s.replace("\n", ""))
        .map(|s| s.chars().collect::<Vec<char>>())
        .map(|mut s| { s.sort(); s.dedup(); s.len()})
        //.inspect(|x| println!("{:#?}", x))
        .sum::<usize>()
}

fn main() -> Result<(), ()> {
    let day: &str = "6";
    aoc_io::init_env();
    let client = aoc_io::build_client().unwrap();
    let input: String = aoc_io::fetch_input_text(&client, day).unwrap();
    //println!("input: {:#?}", input);

    let data: Vec<&str> = input.split_terminator("\n\n").collect();
    //println!("data: {:#?}", data);

    let lvl1 = count_answers_lvl1(data);

    println!("level 1: {:#?}", lvl1);
    //println!("level 2: {:#?}", lvl2);

    //println!("{:#?}", aoc_io::post_result_text(&client, day, "1", lvl1.to_string().as_str()));
    //println!("{:#?}", aoc_io::post_result_text(&client, day, "2", lvl2.to_string().as_str()));

    Ok(())
}

#[test]
fn test_count_answers() {
    let data = vec![
        "abc",
        "a\nb\nc",
        "ab\nac",
        "a\na\na\na",
        "b\n",
    ];

    assert_eq!(count_answers_lvl1(data), 11)
}
