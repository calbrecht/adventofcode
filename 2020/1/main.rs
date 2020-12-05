extern crate aoc_io;

fn main() -> Result<(), ()> {
    aoc_io::init_env();
    let client = aoc_io::build_client().unwrap();
    let input: String = aoc_io::fetch_input_text(&client, "1").unwrap();

    println!("input: {:#?}", input);

    let numbers: Vec<i32> = input.split("\n")
        .filter_map(|x: &str| x.parse::<i32>().ok())
        .collect::<Vec<i32>>();

    let mut lvl1: i32 = 0;
    let mut lvl2: i32 = 0;

    'out: for n1 in &numbers {
        for n2 in &numbers {
            if n1 + n2 == 2020 {
                lvl1 = n1 * n2;
                if lvl2 != 0 {
                    break 'out;
                }
            }
            for n3 in &numbers {
                if n1 + n2 + n3 == 2020 {
                    lvl2 = n1 * n2 * n3;
                    if lvl1 != 0 {
                        break 'out;
                    }
                }
            }
        }
    }

    println!("level 1: {:#?}", lvl1);
    println!("level 2: {:#?}", lvl2);

    //println!("{:#?}", aoc_io::post_result_text(&client, "1", "1", lvl1.to_string().as_str()));
    //println!("{:#?}", aoc_io::post_result_text(&client, "1", "2", lvl2.to_string().as_str()));

    Ok(())
}
