use dialoguer::{Confirm, Select, theme::ColorfulTheme};

use crate::state::App;
use std::time::Instant;

mod state;
mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;


fn main() {
    let app = App::new();

    println!("===============================================================");
    println!(" 🌊 🗝️   Welcome to Ly4m's 2021 - Advent of Code runner ! 🎄 \n");
    println!("===============================================================");


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
            .items(&["part 1", "part 2"])
            .interact()
            .map(|x| x + 1)
            .unwrap();

        match (day, part) {
            (1, 1) => run_bench(day_1::solve_part_1),
            (1, 2) => run_bench(day_1::solve_part_2),
            (2, 1) => run_bench(day_2::solve_part_1),
            (2, 2) => run_bench(day_2::solve_part_2),
            (3, 1) => run_bench(day_3::solve_part_1),
            (3, 2) => run_bench(day_3::solve_part_2),
            (4, 1) => run_bench(day_4::solve_part_1),
            (4, 2) => run_bench(day_4::solve_part_2),
            (5, 1) => run_bench(day_5::solve_part_1),
            (5, 2) => run_bench(day_5::solve_part_2),
            (6, 1) => run_bench_release(day_6::solve_part_1, false),
            (6, 2) => run_bench_release(day_6::solve_part_2, false),
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


fn run_bench_release(f: fn(bool) -> usize, test_mode: bool) {
    println!("\n");
    let start = Instant::now();
    let result = f(test_mode);
    let duration = start.elapsed();
    println!("\n|----------------------------------------");
    println!("|Result: {}", result);
    println!("|Elapsed time: {:?}", duration);
    println!("|----------------------------------------");
}

