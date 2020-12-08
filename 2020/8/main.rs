extern crate aoc_io;

fn main() -> Result<(), ()> {
    let day = "8";
    aoc_io::init_env();
    let client = aoc_io::build_client().unwrap();
    let input: String = aoc_io::fetch_input_text(&client, day).unwrap();
    //println!("input: {:#?}", input);

    let data: &[&str] = &input.split_terminator("\n").collect::<Vec<&str>>()[..];
    //println!("data: {:#?}", data);

    let lvl1: Option<(bool, isize)> = data.iter()
        .enumerate()
        .map(|ins| {
            let cmd: Vec<&str> = ins.1.split(" ").collect();
            match (cmd[0], cmd[1].parse::<isize>()) {
                (a, Ok(b)) => (ins.0 as isize, a, b),
                _ => panic!("ohhh noooooo!")
            }
        })
        .cycle()
        .scan((/*seen*/vec![], /*next*/0, /*acc*/0), |state, cmd| {
            if state.1 == cmd.0 {
                if state.0.contains(&cmd.0) {
                    return Some((false, state.2))
                }
                state.0.push(cmd.0);
                match cmd.1 {
                    "nop" => state.1 = cmd.0 + 1,
                    "acc" => {
                        state.1 = cmd.0 + 1;
                        state.2 = state.2 + cmd.2;
                    },
                    "jmp" => state.1 = cmd.0 + cmd.2,
                    _ => panic!("invalid instruction")
                }
            }
            Some((true, state.2))
        })
        .take_while(|cmd| cmd.0)
        .last();

    let lvl2: Option<(bool, isize)> = data.iter()
        .enumerate()
        .map(|ins| {
            let cmd: Vec<&str> = ins.1.split(" ").collect();
            match (cmd[0], cmd[1].parse::<isize>()) {
                (a, Ok(b)) => (ins.0 as isize, a, b),
                _ => panic!("ohhh noooooo!")
            }
        })
        .cycle()
        .scan((/*seen*/vec![], /*next*/0, /*acc*/0, /*modded*/vec![], /*mod*/false),
              |state, cmd| {
                  if state.1 == cmd.0 {
                      if state.0.contains(&cmd.0) {
                          state.0.clear();
                          state.1 = 0;
                          state.2 = 0;
                          state.4 = false;
                          return Some((true, 0))
                      }
                      state.0.push(cmd.0);
                      match cmd.1 {
                          "nop" => match state.4 || state.3.contains(&cmd.0) {
                              true => state.1 = cmd.0 + 1,
                              false => {
                                  state.1 = cmd.0 + cmd.2;
                                  state.3.push(cmd.0);
                                  state.4 = true;
                              },
                          },
                          "acc" => {
                              state.1 = cmd.0 + 1;
                              state.2 = state.2 + cmd.2;
                          },
                          "jmp" => match state.4 || state.3.contains(&cmd.0) {
                              true => state.1 = cmd.0 + cmd.2,
                              false => {
                                  state.1 = cmd.0 + 1;
                                  state.3.push(cmd.0);
                                  state.4 = true;
                              },
                          },
                          _ => panic!("invalid instruction")
                      }
                  }
                  if state.1 == data.len() as isize {
                      Some((false, state.2))
                  } else {
                      Some((true, state.2))
                  }
        })
        .take_while(|cmd| cmd.0)
        .last();

    println!("level 1: {:#?}", lvl1);
    println!("level 2: {:#?}", lvl2);

    //println!("{:#?}", aoc_io::post_result_text(&client, day, "1", lvl1.to_string().as_str()));
    //println!("{:#?}", aoc_io::post_result_text(&client, day, "2", lvl2.to_string().as_str()));

    Ok(())
}
