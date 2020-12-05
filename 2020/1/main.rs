extern crate aoc_io;

fn main() -> Result<(), ()> {
    aoc_io::init_env();
    let client = aoc_io::build_client().unwrap();
    let input: String = aoc_io::fetch_input_text(&client, "1").unwrap();

    println!("input: {:#?}", input);

    Ok(())
}
