use std::thread::sleep_ms;
use std::env::args;

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

fn total_to_min_sec(total: u32) -> (u32, u32) {
    let min = total / 60;
    let sec = total - min * 60;

    return (min, sec);
}

fn main() {
    let mut options = args().skip(1);

    let first_arg: Option<String> = options.next();
    let minutes = parse_arg(first_arg);
    let second_arg: Option<String> = options.next();
    let seconds = parse_arg(second_arg);

    let mut total = minutes * 60 + seconds;
    let (min, sec) = total_to_min_sec(total);

    loop {
        for counter in (1..=total).rev() {
            let (min, sec) = total_to_min_sec(counter);
            print!("{}{}{}{}{}{}count down: ",
                ESC, CLEAR,
                ESC, HIDE_CURSOR,
                ESC, RESET_CURSOR
            );
            println!("{:02}:{:02}", min, sec);
            sleep_ms(1000);
        }
        break;
    }
}
