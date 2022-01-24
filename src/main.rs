use std::thread::sleep;
use std::env::args;
use std::time::Duration;

static ESC: &str = "\x1b[";
static CLEAR: &str = "2J";
static HIDE_CURSOR: &str = "?25l";
static RESET_CURSOR: &str = "H";

fn parse_arg(arg: Option<String>) -> u32 {
    match arg {
        Some(val) => {
            match val.parse() {
                Ok(number) => return number,
                Err(error) => { 
                    println!("error: {}", error);
                    return 0;
                }
            }
        }
        None => {
            return 0;
        }
    }
}

pub(crate) fn total_to_min_sec(total: u32) -> (u32, u32) {
    let _min = total / 60;
    let _sec = total - _min * 60;

    return (_min, _sec);
}

fn main() {
    let mut options = args().skip(1);

    let first_arg: Option<String> = options.next();
    let minutes = parse_arg(first_arg);
    let second_arg: Option<String> = options.next();
    let seconds = parse_arg(second_arg);

    let total = minutes * 60 + seconds;
    let (_min, _sec): (u32, u32) = total_to_min_sec(total);

    loop {
        for counter in (1..=total).rev() {
            let (min, sec) = total_to_min_sec(counter);
            print!("{}{}{}{}{}{}count down: ",
                ESC, CLEAR,
                ESC, HIDE_CURSOR,
                ESC, RESET_CURSOR
            );
            println!("{:02}:{:02}", min, sec);
            sleep(Duration::from_millis(1000 as u64));
        }
        break;
    }
}
