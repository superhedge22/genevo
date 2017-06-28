//! The `order` module provides `operator::MutationOp`s for permutation encoded
//! `genetic::Genotype`s.

use operator::{GeneticOperator, MutationOp};
use simulation::SimError;
use random::random_cut_points;
use rand::Rng;
use std::fmt::Debug;


#[derive(Clone)]
pub struct InsertOrderMutator {
    mutation_rate: f64,
}

impl InsertOrderMutator {
    pub fn new(mutation_rate: f64) -> Self {
        InsertOrderMutator {
            mutation_rate: mutation_rate,
        }
    }

    pub fn mutation_rate(&self) -> f64 {
        self.mutation_rate
    }

    pub fn set_mutation_rate(&mut self, value: f64) {
        self.mutation_rate = value;
    }
}

impl GeneticOperator for InsertOrderMutator {
    fn name() -> String {
        "Order-Insert-Mutation".to_string()
    }
}

impl<V> MutationOp<Vec<V>> for InsertOrderMutator
    where V: Clone + Debug + PartialEq {

    fn mutate<R>(&self, genome: Vec<V>, rng: &mut R)
        -> Result<Vec<V>, SimError>
        where R: Rng + Sized {
        let genome_length = genome.len();
        let num_mutations = ((genome_length as f64 * self.mutation_rate) + rng.next_f64()).floor() as usize;
        let mut mutated = genome;
        for _ in 0..num_mutations {
            let (locus1, locus2) = random_cut_points(rng, genome_length);
            let value2 = mutated.remove(locus2);
            mutated.insert(locus1 + 1, value2);
        }
        Ok(mutated)
    }
}

#[derive(Clone)]
pub struct SwapOrderMutator {
    mutation_rate: f64,
}

impl SwapOrderMutator {
    pub fn new(mutation_rate: f64) -> Self {
        SwapOrderMutator {
            mutation_rate: mutation_rate,
        }
    }

    pub fn mutation_rate(&self) -> f64 {
        self.mutation_rate
    }

    pub fn set_mutation_rate(&mut self, value: f64) {
        self.mutation_rate = value;
    }
}

impl GeneticOperator for SwapOrderMutator {
    fn name() -> String {
        "Order-Swap-Mutation".to_string()
    }
}

impl<V> MutationOp<Vec<V>> for SwapOrderMutator
    where V: Clone + Debug + PartialEq {

    fn mutate<R>(&self, genome: Vec<V>, rng: &mut R)
        -> Result<Vec<V>, SimError>
        where R: Rng + Sized {
        let genome_length = genome.len();
        let num_mutations = ((genome_length as f64 * self.mutation_rate) + rng.next_f64()).floor() as usize;
        let mut mutated = genome;
        for _ in 0..num_mutations {
            let (locus1, locus2) = random_cut_points(rng, genome_length);
            mutated.swap(locus1, locus2);
        }
        Ok(mutated)
    }
}
