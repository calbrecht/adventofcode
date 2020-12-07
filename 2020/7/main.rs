extern crate aoc_io;

struct Bag {
    style: String,
    holds: Vec<(usize, String)>,
}

impl Bag {
    fn can_hold(&self, style: String) -> bool {
        self.holds.iter().any(|bag| bag.1 == style)
    }
}

impl From<&&str> for Bag {
    fn from(s: &&str) -> Self {
        let m: Vec<&str> = s.split(" bags contain ").collect();
        let holds: Vec<(usize, String)> = m[1].split_terminator(&['.', ','][..])
            .map(|s| s.trim().replace(" bags", "").replace(" bag", ""))
            .map(|s| match s.starts_with(|c: char| c.is_ascii_digit()) {
                true => {
                    let p: Vec<&str> = s.split(" ").collect();
                    (p[0].parse::<usize>().unwrap(), p[1..].join(" "))
                },
                false => (0, s.to_string())
            })
            .collect();

        Bag {
            style: String::from(m[0]),
            holds
        }
    }
}

fn parse_lvl1(v: &[&str]) -> Vec<Bag> {
    v.iter().map(Bag::from).collect()
}

fn count_bags_lvl1<'a >(bags: &'a [Bag], styles: &[&'a str]) -> Vec<&'a str> {

    let found: Vec<&str> = bags.iter()
        .filter(|bag| styles.iter().any(|style| bag.can_hold(style.to_string())))
        .map(|bag| bag.style.as_str())
        .collect();

    let mut all = Vec::from(found.clone());

    match found.len() {
        0 => all,
        _ => {
            all.append(&mut count_bags_lvl1(&bags, &found[..]));
            all.sort();
            all.dedup();
            all
        }
    }
}

fn main() -> Result<(), ()> {
    let day = "7";
    aoc_io::init_env();
    let client = aoc_io::build_client().unwrap();
    let input: String = aoc_io::fetch_input_text(&client, day).unwrap();
    //println!("input: {:#?}", input);

    let data: &[&str] = &input.split_terminator("\n").collect::<Vec<&str>>()[..];
    //println!("data: {:#?}", data);

    let lvl1_bags = parse_lvl1(&data);
    let lvl1 = count_bags_lvl1(&lvl1_bags, &["shiny gold"]);

    println!("level 1: {:#?}", lvl1.len());
    //println!("level 2: {:#?}", lvl2);

    //println!("{:#?}", aoc_io::post_result_text(&client, day, "1", lvl1.to_string().as_str()));
    //println!("{:#?}", aoc_io::post_result_text(&client, day, "2", lvl2.to_string().as_str()));

    Ok(())
}
