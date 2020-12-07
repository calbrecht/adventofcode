extern crate aoc_io;

fn count_chars(s: &String) -> Vec<(char, usize)> {
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort();
    chars.dedup();
    chars.iter()
        .map(|c| (c.clone(), s.matches(|a| a == *c).count()))
        .collect()
}

fn count_answers_lvl1(v: &Vec<&str>) -> usize {
    v.iter()
        .map(|s| s.replace("\n", ""))
        .map(|s| s.chars().collect::<Vec<char>>())
        .map(|mut s| { s.sort(); s.dedup(); s.len()})
        //.inspect(|x| println!("{:#?}", x))
        .sum::<usize>()
}

fn count_answers_lvl2(v: &Vec<&str>) -> usize {
    v.iter()
        .map(|s| s.split_terminator("\n").collect::<Vec<&str>>())
        .map(|s| (s.len(), s.join("")))
        .map(|s| count_chars(&s.1).iter().filter(|c|c.1 == s.0).count())
        //.inspect(|x| println!("{:#?}", x))
        .sum()
}

fn main() -> Result<(), ()> {
    let day: &str = "6";
    aoc_io::init_env();
    let client = aoc_io::build_client().unwrap();
    let input: String = aoc_io::fetch_input_text(&client, day).unwrap();
    //println!("input: {:#?}", input);

    let data: Vec<&str> = input.split_terminator("\n\n").collect();
    //println!("data: {:#?}", data);

    let lvl1 = count_answers_lvl1(&data);
    let lvl2 = count_answers_lvl2(&data);

    println!("level 1: {:#?}", lvl1);
    println!("level 2: {:#?}", lvl2);

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

    assert_eq!(count_answers_lvl1(&data), 11);
    assert_eq!(count_answers_lvl2(&data), 6);
}
