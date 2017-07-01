//! The `monkeys` example explores the idea of the Shakespeare's monkeys or the
//! [infinite monkey theorem](https://en.wikipedia.org/wiki/Infinite_monkey_theorem).

extern crate genevo;
extern crate rand;

use genevo::genetic::{FitnessEvaluation, PopulationGenerator};
use genevo::mutation::value::RandomValueMutator;
use genevo::recombination::discrete::MultiPointCrossBreeder;
use genevo::reinsertion::elitist::ElitistReinserter;
use genevo::selection::truncation::MaximizeSelector;
use genevo::simulation::{Simulation, SimulationBuilder, SimResult};
use genevo::simulation::ga;
use genevo::termination::or;
use genevo::termination::limiter::{FitnessLimit, GenerationLimit};
use genevo::types::Display;
use rand::{Rng, thread_rng};

//const TARGET_TEXT: &str = "See how a genius creates a legend";
const TARGET_TEXT: &str = "Be not afraid of greatness! Some are great, some achieve greatness, \
                           and some have greatness thrust upon 'em.";
//const TARGET_TEXT: &str = "All the world's a stage, and all the men and women merely players: \
//                           they have their exits and their entrances; and one man in his time \
//                           plays many parts, his acts being seven ages.";

#[derive(Debug)]
struct Parameter {
    population_size: usize,
    generation_limit: u64,
    num_individuals_per_parents: usize,
    selection_ratio: f64,
    num_crossover_points: usize,
    mutation_rate: f64,
    reinsertion_ratio: f64,
}

impl Default for Parameter {
    fn default() -> Self {
        Parameter {
            population_size: (100. * (TARGET_TEXT.len() as f64).ln()) as usize,
            generation_limit: 2000,
            num_individuals_per_parents: 2,
            selection_ratio: 0.7,
            num_crossover_points: TARGET_TEXT.len() / 6,
            mutation_rate: 0.05 / (TARGET_TEXT.len() as f64).ln(),
            reinsertion_ratio: 0.7,
        }
    }
}

/// The phenotype
type Text = String;

/// The genotype
type TextGenome = Vec<u8>;

/// How do the genes of the genotype show up in the phenotype
trait AsPhenotype {
    fn as_text(&self) -> Text;
}

impl AsPhenotype for TextGenome {
    fn as_text(&self) -> Text {
        String::from_utf8(self.to_vec()).unwrap()
    }
}

/// The fitness function for `TextGenome`s.
#[derive(Clone)]
struct FitnessCalc {}

impl FitnessEvaluation<TextGenome, usize> for FitnessCalc {

    fn fitness_of(&self, genome: &TextGenome) -> usize {
        let mut score = 0;
        for (c, t) in genome.iter().zip(TARGET_TEXT.chars()) {
            let c = *c as char;
            if c == t {
                score += 1;
            }
        }
        let fraction = score as f32 / TARGET_TEXT.len() as f32;
        (fraction * fraction * 100_00. + 0.5).floor() as usize
    }

    fn average(&self, fitness_values: &[usize]) -> usize {
        fitness_values.iter().sum::<usize>() / fitness_values.len()
    }

    fn highest_possible_fitness(&self) -> usize {
        100_00
    }

    fn lowest_possible_fitness(&self) -> usize {
        0
    }
}

/// The random population generator
struct Monkey {}

impl PopulationGenerator<TextGenome> for Monkey {
    fn generate_genotype<R>(&self, rng: &mut R) -> TextGenome
        where R: Rng + Sized {
        (0..TARGET_TEXT.len()).map(|_|
            rng.gen_range(32u8, 126u8)
        ).collect()
    }
}

fn main() {

    let params = Parameter::default();

    let mut rng = thread_rng();

    let initial_population = Monkey{}.generate_population(params.population_size, &mut rng);

    let mut monkeys_sim = ga::Simulator::builder(
        FitnessCalc {},
        MaximizeSelector::new(params.selection_ratio, params.num_individuals_per_parents),
        MultiPointCrossBreeder::new(params.num_crossover_points),
        RandomValueMutator::new(params.mutation_rate, 32, 126),
        ElitistReinserter::new(FitnessCalc{}, true, params.reinsertion_ratio),
        or(FitnessLimit::new(FitnessCalc{}.highest_possible_fitness()),
           GenerationLimit::new(params.generation_limit))
    ).initialize(initial_population);

    println!("Starting Shakespeare's Monkeys with: {:?}", params);

    loop {
        let result = monkeys_sim.step();
        match result {
            Ok(SimResult::Intermediate(result)) => {
                println!("Step: generation: {}, average_fitness: {}, \
                         best_solution: [{}], fitness: {}, processing_time: {}",
                         result.generation, result.average_fitness,
                         result.best_solution.solution.genome.as_text(),
                         result.best_solution.solution.fitness,
                         result.processing_time.fmt());
//                println!("| population: [{}]", result.population.iter().map(|g| g.as_text())
//                    .collect::<Vec<String>>().join("], ["));
            },
            Ok(SimResult::Final(result, duration, stop_reason)) => {
                println!("{}", stop_reason);
                println!("Final result after {}: generation: {}, \
                         best_solution: [{}] with fitness {} found in generation {}, processing_time: {}",
                         duration.fmt(), result.generation,
                         result.best_solution.solution.genome.as_text(),
                         result.best_solution.solution.fitness,
                         result.best_solution.generation,
                         result.processing_time.fmt());
                break;
            },
            Err(error) => {
                println!("{:?}", error);
                break;
            },
        }
    }
}
