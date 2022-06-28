
use std::fmt::Display;
use rand::prelude::*;

pub type FitnessMeasure = f32;

pub trait Fitness {
    fn evaluate(&self) -> FitnessMeasure;
}

pub trait CanMutate {
    fn mutate(&mut self);
}

pub trait CanReproduce {
    type Partner: CanReproduce;
    type Child: CanReproduce;

    fn reproduce(&self, partner: &Self::Partner) -> Self::Child;
}


#[derive(Debug)]
pub struct MuLambdaEvolution<P: Fitness + CanMutate + CanReproduce> {
    pub generation: i32,
    pub population: Vec<P>,
    pub elite_rate: u32,
    pub reproduction_rate: u32,
}

impl<P: Fitness + CanMutate + CanReproduce + Default + Clone + Display> MuLambdaEvolution<P> {
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
            .sort_by(|(_, fit_left), (_, fit_right)| fit_left.partial_cmp(fit_right).unwrap());

        // step 5: remove lambda lowest-scored
        population_with_fitness
            .rotate_right(self.reproduction_rate as usize);
        population_with_fitness
            .truncate(self.reproduction_rate as usize);

        // step 6: make offspring
        let offspring : Vec<(P, FitnessMeasure)> =
            (1..self.elite_rate)
            .map(|i| population_with_fitness.get(i as usize).unwrap().clone())
            .collect();

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