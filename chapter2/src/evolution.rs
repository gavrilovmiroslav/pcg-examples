use std::cmp::Ordering;
use std::fmt::Display;
use rand::prelude::*;

pub type FitnessMeasure = f32;

pub trait Fitness {
    fn evaluate(&self) -> FitnessMeasure;
}

pub trait CanMutate {
    fn mutate(&mut self);
}

#[derive(Debug)]
pub struct MuLambdaEvolution<P: Fitness + CanMutate> {
    pub generation: i32,
    pub population: Vec<P>,
    pub elite_rate: u32,            // MU
    pub reproduction_rate: u32,     // LAMBDA
}

impl<P: Fitness + CanMutate + Default + Clone + Display> MuLambdaEvolution<P> {
    pub fn with_population(mu: u32, lambda: u32) -> Self {
        Self {
            generation: 1,
            // step 1: create population
            population: (1..(mu + lambda)).map(|_| P::default()).collect(),
            elite_rate: mu,
            reproduction_rate: lambda,
        }
    }

    pub fn evolve_until(&mut self, success: fn(i32, &P) -> bool, debug_print: bool) {
        let mut done = false;

        while !done {
            if debug_print {
                self.print();
            }
            self.evolve_step();
            done = self.population.iter()
                .map(|member| success(self.generation, member))
                .fold(false, |sum, next| { sum | next });
        }
    }

    pub fn evolve_step(&mut self) {
        self.generation += 1;

        // step 2: shuffle the population to escape loss-of-gradient situations
        self.population.shuffle(&mut thread_rng());

        // step 3: get fitness for population
        let mut population_with_fitness : Vec<(P, FitnessMeasure)> =
            self.population.iter()
                .map(|c| (c.clone(), c.evaluate()))
                .collect();

        // step 4: sort by fitness
        population_with_fitness
            .sort_by(|(_, fit_left), (_, fit_right)|
                fit_right.partial_cmp(fit_left).unwrap_or(Ordering::Equal));

        // step 5: remove lambda lowest-scored
        population_with_fitness
            .truncate(self.reproduction_rate as usize);

        // step 6: make offspring
        let offspring : Vec<(P, FitnessMeasure)> =
            population_with_fitness.iter()
                .take(self.elite_rate as usize)
                .cloned().collect();

        // step 7: mutate offspring and add them back into the population
        for (mut member, fitness) in offspring {
            member.mutate();
            population_with_fitness.push((member, fitness));
        }

        // step 8: replace the population with the new one
        self.population = population_with_fitness.iter()
            .map(|(member, _)| member.clone())
            .collect();
    }

    pub fn print(&self) {
        print!("{}: ", self.generation);
        for member in &self.population {
            print!("[{}]({})  ", member, member.evaluate());
        }
        println!();
    }
}