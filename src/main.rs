mod display;
mod gol;
mod help;

use display::dynamic_display;
use std::env;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

static ARGUMENT_PARSE_ERROR: &str = "Could not parse argument";

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut width: u16 = 20;
    let mut height: u16 = 20;
    let mut delay: u64 = 500;
    let mut alive_cell: char = 'O';
    let mut dead_cell: char = '\'';

    let mut help = false;

    for arg in args.iter().skip(1) {
        if arg.starts_with("width=") {
            let n = arg[6..arg.len()]
                .parse::<u16>()
                .expect(&format!("{} {}", ARGUMENT_PARSE_ERROR, arg));
            width = n;
        } else if arg.starts_with("height=") {
            let n = arg[7..arg.len()]
                .parse::<u16>()
                .expect(&format!("{} {}", ARGUMENT_PARSE_ERROR, arg));
            height = n;
        } else if arg.starts_with("delay=") {
            let n = arg[6..arg.len()]
                .parse::<u64>()
                .expect(&format!("{} {}", ARGUMENT_PARSE_ERROR, arg));
            delay = n;
        } else if arg.starts_with("alive=") {
            let n = arg[6..arg.len()]
                .parse::<char>()
                .expect(&format!("{} {}", ARGUMENT_PARSE_ERROR, arg));
            alive_cell = n;
        } else if arg.starts_with("dead=") {
            let n = arg[5..arg.len()]
                .parse::<char>()
                .expect(&format!("{} {}", ARGUMENT_PARSE_ERROR, arg));
            dead_cell = n;
        } else if arg == "help" {
            help = true;
        }
        else {
            panic!(format!("{} {}", ARGUMENT_PARSE_ERROR, arg));
        }
    }

    if help {
        help::show_help();
    } else {
        let will_continue = Arc::new(AtomicBool::new(true));

        let mut game = gol::new_game(width, height);
        let mut display = dynamic_display::new(alive_cell, dead_cell, delay);
        game.randomize();
        display.start(&mut game, &will_continue);
    }
}
