use std::{thread, time};
use crate::smartpointers::universe;

mod lifetime;
mod common;
mod smartpointers;

fn print_universe(universe: &universe::Universe) {
    for line_to_print in universe.print() {
        println!("{:?}", line_to_print);
    }
}

fn main() {
    let universe = universe::Universe::new(10, 10);
    let number_of_ticks = 10;
    let sleep_duration = time::Duration::from_millis(500);

    print_universe(&universe);
    for _ in 0..=number_of_ticks {
        thread::sleep(sleep_duration);
        universe.tick();
        print_universe(&universe);
        println!("\n");
    }
}
