extern crate aoc_io;

use std::iter::FromIterator;

struct Rule {
    bag: String,
    contents: String,
}

impl<'a> FromIterator<&'a str> for Rule {
    fn from_iter<I: IntoIterator<Item = &'a str>>(iter: I) -> Self {
        let mut iterator = iter.into_iter();

        Self {
            bag: iterator.next().unwrap_or("").to_string(),
            contents: iterator.next().unwrap_or("").to_string(),
        }
    }
}

impl From<&str> for Rule {
    fn from(s: &str) -> Self {
        s.split(" bags contain ").collect::<Rule>()
    }
}

impl From<&&str> for Rule {
    fn from(s: &&str) -> Self {
        Rule::from(*s)
    }
}

struct Bags {
    style: String,
    _count: usize,
}

impl From<&str> for Bags {
    fn from(s: &str) -> Self {
        let sentence = s.trim().replace(" bags", "").replace(" bag", "");

        let (_count, style) = match sentence.starts_with(|c: char| c.is_ascii_digit()) {
            true => {
                let p: Vec<&str> = sentence.split(" ").collect();
                (p[0].parse::<usize>().unwrap(), p[1..].join(" "))
            },
            false => (0, sentence.to_string())
        };

        Bags {
            style,
            _count
        }
    }
}

struct Content {
    bags: Vec<Bags>
}

impl From<Rule> for Content {
    fn from(rule: Rule) -> Self {
        Self {
            bags: rule.contents.split_terminator(&['.', ','][..]).map(Bags::from).collect()
        }
    }
}

struct Bag {
    style: String,
    content: Content,
}

impl Bag {
    fn can_hold(&self, style: &str) -> bool {
        self.content.bags.iter().any(|inner| inner.style == style)
    }
}

impl From<Rule> for Bag {
    fn from(rule: Rule) -> Self {
        Self {
            style: rule.bag.clone(),
            content: Content::from(rule)
        }
    }
}

fn count_bags_lvl1<'a >(bags: &'a [Bag], styles: &[&'a str]) -> Vec<&'a str> {

    let found: Vec<&str> = bags.iter()
        .filter(|bag| styles.iter().any(|style| bag.can_hold(style)))
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

    let bags: Vec<Bag> = data.iter().map(Rule::from).map(Bag::from).collect();

    let lvl1 = count_bags_lvl1(&bags, &["shiny gold"]);

    println!("level 1: {:#?}", lvl1.len());
    //println!("level 2: {:#?}", lvl2);

    //println!("{:#?}", aoc_io::post_result_text(&client, day, "1", lvl1.to_string().as_str()));
    //println!("{:#?}", aoc_io::post_result_text(&client, day, "2", lvl2.to_string().as_str()));

    Ok(())
}
