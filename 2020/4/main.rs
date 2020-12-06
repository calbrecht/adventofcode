extern crate aoc_io;

use std::collections::HashMap;

#[derive(Debug)]
struct Passport {
    pub byr: String, //(Birth Year)
    pub cid: String, //(Country ID)
    pub ecl: String, //(Eye Color)
    pub eyr: String, //(Expiration Year)
    pub hcl: String, //(Hair Color)
    pub hgt: String, //(Height)
    pub iyr: String, //(Issue Year)
    pub pid: String, //(Passport ID)
}

impl From<&&str> for Passport {

    fn from(s: &&str) -> Self {

        let m: HashMap<&str, &str> =
            s.split_terminator(&['\n', ' ', ':'][..])
            .collect::<Vec<&str>>()
            .chunks(2)
            .fold(HashMap::new(), |mut acc, a: &[&str]| {acc.insert(a[0], a[1]); acc});

        Passport {
            byr: m.get("byr").unwrap_or(&&"N/A").to_string(),
            cid: m.get("cid").unwrap_or(&&"N/A").to_string(),
            ecl: m.get("ecl").unwrap_or(&&"N/A").to_string(),
            eyr: m.get("eyr").unwrap_or(&&"N/A").to_string(),
            hcl: m.get("hcl").unwrap_or(&&"N/A").to_string(),
            hgt: m.get("hgt").unwrap_or(&&"N/A").to_string(),
            iyr: m.get("iyr").unwrap_or(&&"N/A").to_string(),
            pid: m.get("pid").unwrap_or(&&"N/A").to_string(),
        }
    }
}

fn main() -> Result<(), ()> {
    let day: &str = "4";
    aoc_io::init_env();
    let client = aoc_io::build_client().unwrap();
    let input: String = aoc_io::fetch_input_text(&client, day).unwrap();
    //println!("input: {:#?}", input);

    let data: Vec<&str> = input.split_terminator("\n\n").collect();
    println!("data: {:#?}", data);

    let na = String::from("N/A");

    let lvl1: usize= data.iter()
        .map(Passport::from)
        .filter(|x| x.byr != na
                && x.ecl != na
                && x.eyr != na
                && x.hcl != na
                && x.hgt != na
                && x.iyr != na
                && x.pid != na)
        //.inspect(|x| println!("{:#?}", x))
        .count();

    println!("level 1: {:#?}", lvl1);
    //println!("level 2: {:#?}", lvl2);

    //println!("{:#?}", aoc_io::post_result_text(&client, day, "1", lvl1.to_string().as_str()));
    //println!("{:#?}", aoc_io::post_result_text(&client, day, "2", lvl2.to_string().as_str()));

    Ok(())
}
