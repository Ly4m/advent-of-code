use dialoguer::{Confirm, Select, theme::ColorfulTheme};

use crate::state::App;
use std::time::Instant;

mod state;
mod day_1;
mod day_9;
mod day_10;

fn main() {
    let app = App::new();

    println!("======================================================");
    println!(" 🎄 Welcome to Ly4m's 2020 Advent of Code runner ! \n");

    loop {
        let day = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Pick a day ")
            .default(0)
            .items(&app.days[..])
            .interact()
            .map(|x| x + 1)
            .unwrap();

        let part = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Pick a part ")
            .default(0)
            .items(&vec!["part 1", "part 2"])
            .interact()
            .map(|x| x + 1)
            .unwrap();

        match (day, part) {
            (1, 1) => run_bench(day_1::solve_part_1),
            (1, 2) => run_bench(day_1::solve_part_2),
            (9, 1) => run_bench(day_9::solve_part_1),
            (9, 2) => run_bench(day_9::solve_part_2),
            (10, 1) => run_bench(day_10::solve_part_1),
            (10, 2) => run_bench(day_10::solve_part_2),
            _ => println!("Not yet Implemented")
        }

        if !Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Do you want to run another puzzle ?")
            .interact()
            .unwrap()
        {
            println!("Bye bye 👋 \n");
            break;
        }
    }
}

fn run_bench(f: fn() -> usize) {
    println!("\n");
    let start = Instant::now();
    let result = f();
    let duration = start.elapsed();
    println!("\n|----------------------------------------");
    println!("|Result: {}", result);
    println!("|Elapsed time: {:?}", duration);
    println!("|----------------------------------------");
}

