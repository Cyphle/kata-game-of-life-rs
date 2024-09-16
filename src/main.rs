use std::{thread, time};
use crate::smartpointers::universe as sp;
use crate::nopointer::universe as np;

mod nopointer;
mod common;
mod smartpointers;

fn print_universe(universe: &sp::Universe) {
    for line_to_print in universe.print() {
        println!("{:?}", line_to_print);
    }
}

fn print_universe_np(universe: &np::Universe) {
    for line_to_print in universe.print() {
        println!("{:?}", line_to_print);
    }
}

fn main() {
    println!("With smart pointers");
    let universe = sp::Universe::new(10, 10);
    let number_of_ticks = 10;
    let sleep_duration = time::Duration::from_millis(500);

    print_universe(&universe);
    for round in 0..=number_of_ticks {
        thread::sleep(sleep_duration);
        println!("Tick {}", round);
        universe.tick();
        print_universe(&universe);
        println!("\n");
        print!("{}[2J", 27 as char);
    }

    println!("It started with lifetimes but finally no pointer as to force SRP");
    let mut universe = np::Universe::new(10, 10);
    let number_of_ticks = 10;
    let sleep_duration = time::Duration::from_millis(500);

    print_universe_np(&universe);
    for round in 0..=number_of_ticks {
        thread::sleep(sleep_duration);
        let new_universe = universe.tick();
        println!("Tick {}", round);
        print_universe_np(&universe);
        universe = new_universe;
        println!("\n");
    }
}
