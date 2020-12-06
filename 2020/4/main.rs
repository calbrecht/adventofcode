extern crate aoc_io;

use std::collections::HashMap;

#[derive(Debug)]
struct Passport {
    pub byr: i16, //(Birth Year)
    pub cid: String, //(Country ID)
    pub ecl: String, //(Eye Color)
    pub eyr: i16, //(Expiration Year)
    pub hcl: String, //(Hair Color)
    pub hgt: (i16, String), //(Height)
    pub iyr: i16, //(Issue Year)
    pub pid: String, //(Passport ID)
}

impl Passport {
    pub fn is_valid_lvl1(&self) -> bool {

        let na = String::from("N/A");

        self.byr != 0
            && self.ecl != na
            && self.eyr != 0
            && self.hcl != na
            && self.hgt.0 != 0
            && self.iyr != 0
            && self.pid != na
    }

    pub fn is_valid_lvl2(&self) -> bool {
        // byr (Birth Year) - four digits; at least 1920 and at most 2002.
        // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
        // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
        // hgt (Height) - a number followed by either cm or in:
        //   If cm, the number must be at least 150 and at most 193.
        //   If in, the number must be at least 59 and at most 76.
        // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
        // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
        // pid (Passport ID) - a nine-digit number, including leading zeroes.
        // cid (Country ID) - ignored, missing or not.

        let hex = ['0','1','2','3','4','5','6','7','8','9','a','b','c','d','e','f'];
        let ecl = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

        self.is_valid_lvl1()
            && self.byr >= 1920 && self.byr <= 2002
            && self.iyr >= 2010 && self.iyr <= 2020
            && self.eyr >= 2020 && self.eyr <= 2030
            && match self.hgt.1.as_str() {
                "cm" => self.hgt.0 >= 150 && self.hgt.0 <= 193,
                "in" => self.hgt.0 >= 59 && self.hgt.0 <= 76,
                _ => false,
            }
            && self.hcl.starts_with("#")
            && self.hcl[1..7].matches(&hex[..]).count() == 6
            && ecl.contains(&self.ecl.as_str())
            && self.pid.len() == 9
            && self.pid[..].matches(|x: char| x.is_ascii_digit()).count() == 9
    }
}

impl From<&&str> for Passport {

    fn from(s: &&str) -> Self {

        let m: HashMap<&str, &str> =
            s.split_terminator(&['\n', ' ', ':'][..])
            .collect::<Vec<&str>>()
            .chunks(2)
            .fold(HashMap::new(), |mut acc, a: &[&str]| {acc.insert(a[0], a[1]); acc});

        let parse_hgt = |s: &str|
        (s.matches(|x: char| x.is_ascii_digit()).collect::<String>().parse::<i16>().unwrap_or(0),
         s.rmatches(|x: char| !x.is_ascii_digit()).rev().collect::<String>());

        Passport {
            byr: m.get("byr").unwrap_or(&"0").parse::<i16>().unwrap_or(0),
            cid: m.get("cid").unwrap_or(&"N/A").to_string(),
            ecl: m.get("ecl").unwrap_or(&"N/A").to_string(),
            eyr: m.get("eyr").unwrap_or(&"0").parse::<i16>().unwrap_or(0),
            hcl: m.get("hcl").unwrap_or(&"N/A").to_string(),
            hgt: parse_hgt(m.get("hgt").unwrap_or(&"0")),
            iyr: m.get("iyr").unwrap_or(&"0").parse::<i16>().unwrap_or(0),
            pid: m.get("pid").unwrap_or(&"N/A").to_string(),
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

    let lvl1: usize= data.iter()
        .map(Passport::from)
        //.inspect(|x| println!("{:#?}", x))
        .filter(|x| x.is_valid_lvl1())
        .count();

    let lvl2: usize= data.iter()
        .map(Passport::from)
        //.inspect(|x| println!("{:#?}", x))
        .filter(|x| x.is_valid_lvl2())
        .count();

    println!("level 1: {:#?}", lvl1);
    println!("level 2: {:#?}", lvl2);

    //println!("{:#?}", aoc_io::post_result_text(&client, day, "1", lvl1.to_string().as_str()));
    //println!("{:#?}", aoc_io::post_result_text(&client, day, "2", lvl2.to_string().as_str()));

    Ok(())
}
