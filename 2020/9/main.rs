extern crate aoc_io;

fn main() -> Result<(), ()> {
    let day = "9";
    aoc_io::init_env();
    let client = aoc_io::build_client().unwrap();
    let input: String = aoc_io::fetch_input_text(&client, day).unwrap();

    let data: &[(usize, usize)] = &input.split_terminator("\n")
        .map(|s| s.parse::<usize>().unwrap())
        .enumerate()
        .collect::<Vec<(usize, usize)>>()[..];

    let window = 25;

    let lvl1: &(usize, usize) = data.iter()
        .skip(window)
        .skip_while(|x|
                    data.iter().skip(x.0 - window).take(window)
                    .any(|a|
                         data.iter().skip(x.0 - (window - 1)).take(window - 1)
                         .any(|b|
                              &a.1 + &b.1  == x.1)))
        .take(1)
        .collect::<Vec<&(usize, usize)>>()
        .first()
        .unwrap();

    println!("level 1: at index {:#?} number {:#?}", lvl1.0, lvl1.1);

    Ok(())
}

/*

35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576


*/
