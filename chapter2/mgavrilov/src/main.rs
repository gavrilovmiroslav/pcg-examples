use std::cmp::Ordering;
use crate::creature::Creature;
use crate::evolution::{Fitness, FitnessMeasure, MuLambdaEvolution};
use crate::linecraft::LinecraftMap;

mod evolution;
mod creature;
mod linecraft;

fn run_creature_example() {
    let mut ev = MuLambdaEvolution::<Creature>::with_population(5, 5);
    ev.evolve_until(|t, p| t >= 100 || p.evaluate() > 1000.0, true);
    ev.print();
}

fn run_map_example(mu: u32, lambda: u32) {
    fn map_evaluator(t: i32, map: &LinecraftMap) -> bool {
        t >= 1000 || map.evaluate() > 200.0
    }

    let mut ev = MuLambdaEvolution::<LinecraftMap>::with_population(mu, lambda);
    ev.evolve_until(map_evaluator, true);
    ev.print();

    let best_map = ev.population.iter().max_by(|mapa, mapb|
        mapa.evaluate().partial_cmp(&mapb.evaluate()).unwrap_or(Ordering::Equal)).unwrap();

    println!("Best map: {} (value = {})", best_map, best_map.evaluate());
}

fn main() {
    println!("Creatures example: ");
    run_creature_example();

    println!();

    println!("Linecraft example: ");
    run_map_example(17, 5);
}
