//! darwin-rs: evolutionary algorithms with Rust
//!
//! Written by Willi Kappler, Version 0.2 (2016.07.xx)
//!
//! Repository: https://github.com/willi-kappler/darwin-rs
//!
//! License: MIT
//!
//! This library allows you to write evolutionary algorithms (EA) in Rust.
//! Examples provided: TSP, Sudoku, Queens Problem
//!
//!

use std;

use simulation::{Simulation, SimulationType, SimulationResult};
use individual::{Individual};
use population::Population;

/// This is a helper struct in order to build (configure) a valid simulation.
/// See builder pattern: https://en.wikipedia.org/wiki/Builder_pattern
pub struct SimulationBuilder<S: Send + Sync, T: Individual + Send + Sync> {
    /// The actual simulation
    simulation: Simulation<S, T>,
}

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        /// The number of iteration is too low, should be >= 10
        EndIterationTooLow {}
    }
}

pub type Result<S, T> = std::result::Result<Simulation<S, T>, Error>;

/// This implementation contains all the helper method to build (configure) a valid simulation
impl<S: Send + Sync, T: Individual + Send + Sync> SimulationBuilder<S, T> {
    /// Start with this method, it must always be called as the first one.
    /// It creates a default simulation with some dummy (but invalid) values.
    pub fn new() -> SimulationBuilder<S, T> {
        SimulationBuilder {
            simulation: Simulation {
                type_of_simulation: SimulationType::EndIteration(10),
                num_of_threads: 2,
                habitat: Vec::new(),
                total_time_in_ms: 0.0,
                simulation_result: SimulationResult {
                    improvement_factor: std::f64::MAX,
                    original_fitness: std::f64::MAX,
                    fittest: Vec::new(),
                    iteration_counter: 0
                }
            },
        }
    }

    /// Set the total number of iterations for the simulation and thus sets the simulation
    /// type to `EndIteration`. (Only usefull in combination with `EndIteration`).
    pub fn iterations(mut self, iterations: u32) -> SimulationBuilder<S, T> {
        self.simulation.type_of_simulation = SimulationType::EndIteration(iterations);
        self
    }

    /// Set the improvement factor stop criteria for the simulation and thus sets the simulation
    /// type to `EndFactor`. (Only usefull in combination with `EndFactor`).
    pub fn factor(mut self, factor: f64) -> SimulationBuilder<S, T> {
        self.simulation.type_of_simulation = SimulationType::EndFactor(factor);
        self
    }

    /// Set the minimum fitness stop criteria for the simulation and thus sets the simulation
    /// type to `EndFitness`. (Only usefull in combination with `EndFactor`).
    pub fn fitness(mut self, fitness: f64) -> SimulationBuilder<S, T> {
        self.simulation.type_of_simulation = SimulationType::EndFitness(fitness);
        self
    }

    /// Sets the number of threads in order to speed up the simulation.
    pub fn threads(mut self, threads: usize) -> SimulationBuilder<S, T> {
        self.simulation.num_of_threads = threads;
        self
    }

    /// Add a population to the simulation.
    pub fn add_population(mut self, population: Population<S, T>) -> SimulationBuilder<S, T> {
        self.simulation.habitat.push(population);
        self
    }

    /// This checks the configuration of the simulation and returns an error or Ok if no errors
    /// where found.
    pub fn finalize(self) -> Result<S, T> {
        match self.simulation {
            Simulation { type_of_simulation: SimulationType::EndIteration(0...9), .. } => {
                Err(Error::EndIterationTooLow)
            }
            _ => Ok(self.simulation),
        }
    }
}
