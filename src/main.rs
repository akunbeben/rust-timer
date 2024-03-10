use crate::timer::Timer;
use mouse_position::mouse_position::Mouse;

mod timer;

fn main() {
    let tm = Timer::start_timer();

    while tm.start_time.is_some() {
        let position = Mouse::get_mouse_position();
        match position {
            Mouse::Position { x, y } => print!("\rx: {}, y: {}", x, y),
            Mouse::Error => print!("\rError getting mouse position"),
       }
    }
}
