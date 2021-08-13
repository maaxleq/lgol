static SCREEN_WRITE_ERROR: &str = "Could not write to screen";

mod print_game {
    use crate::gol;
    use std::io::Write;

    pub fn print(screen: &mut Write, game: &gol::Game, alive_mark: char, dead_mark: char, lines_after: u16) {
        for y in 0..game.get_main_grid().get_size_y() {
            for x in 0..game.get_main_grid().get_size_x() {
                if game.get_main_grid().get_tiles()[x as usize][y as usize].get_state()
                    == gol::TileState::ALIVE
                {
                    write!(screen, "{}", alive_mark).expect(crate::display::SCREEN_WRITE_ERROR);
                } else {
                    write!(screen, "{}", dead_mark).expect(crate::display::SCREEN_WRITE_ERROR);
                }
            }
            writeln!(screen, "").expect(crate::display::SCREEN_WRITE_ERROR);
        }
        for _ in 0..lines_after {
            writeln!(screen, "").expect(crate::display::SCREEN_WRITE_ERROR);
        }
    }
}

pub mod static_display {
    use crate::display::print_game;
    use crate::gol;
    use std::io::stdout;

    pub struct StaticDisplay {
        alive_mark: char,
        dead_mark: char,
        lines_after: u16,
    }

    pub fn new(alive_mark: char, dead_mark: char, lines_after: u16) -> StaticDisplay {
        return StaticDisplay {
            alive_mark: alive_mark,
            dead_mark: dead_mark,
            lines_after: lines_after,
        };
    }

    impl StaticDisplay {
        pub fn display(&self, game: &gol::Game) {
            print_game::print(&mut stdout(), game, self.alive_mark, self.dead_mark, self.lines_after);
        }
    }
}

pub mod dynamic_display {
    use crate::display::print_game;
    use crate::gol;
    use std::{thread, time};
    use std::io::{Write, stdout};
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;
    use termion;

    static SIG_HANDLER_ERROR: &str = "Could not set signal handler";

    pub struct DynamicDisplay {
        alive_mark: char,
        dead_mark: char,
        ms_between_frames: u64,
    }

    pub fn new(alive_mark: char, dead_mark: char, ms_between_frames: u64) -> DynamicDisplay {
        return DynamicDisplay {
            alive_mark: alive_mark,
            dead_mark: dead_mark,
            ms_between_frames: ms_between_frames,
        };
    }

    impl DynamicDisplay {
        pub fn start(&mut self, game: &mut gol::Game, will_continue: &Arc<AtomicBool>) {
            let atomic = will_continue.clone();

            let mut screen = termion::screen::AlternateScreen::from(stdout());

            ctrlc::set_handler(move || {
                atomic.store(false, Ordering::SeqCst);
            }).expect(SIG_HANDLER_ERROR);

            while will_continue.load(Ordering::SeqCst) {
                write!(screen, "{}{}", termion::clear::All, termion::cursor::Goto(1, 1)).expect(crate::display::SCREEN_WRITE_ERROR);
                print_game::print(&mut screen, game, self.alive_mark, self.dead_mark, 0);
                screen.flush().unwrap();
                thread::sleep(time::Duration::from_millis(self.ms_between_frames));
                game.next();
            }
        }
    }
}
