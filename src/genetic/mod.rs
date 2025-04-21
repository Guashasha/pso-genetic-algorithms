use std::{collections::HashMap, ops::RangeInclusive};

use rand::random_range;

use crate::{evaluate, EvolutionStats, MAX_EVUALUATIONS, POPULATION_SIZE};

struct Individual {
    values: Vec<f64>,
    aptitude: f64,
}

impl Individual {
    fn from_population(population: &[Vec<f64>]) -> Vec<Self> {
        population
            .iter()
            .map(|individual| Individual {
                values: individual.clone(),
                aptitude: evaluate(individual),
            })
            .collect()
    }
}

pub fn evolve(
    population: &mut [Vec<f64>],
    constraints: HashMap<usize, RangeInclusive<f64>>,
) -> EvolutionStats {
    let mut stats = EvolutionStats::new();
    let mut population: Vec<Individual> = Individual::from_population(population);
    let mut current_evaluations = POPULATION_SIZE;

    while current_evaluations < MAX_EVUALUATIONS {
        population.sort_by(|a, b| a.aptitude.partial_cmp(&b.aptitude).unwrap());
        stats.best.push(population.first().unwrap().aptitude);
        stats
            .middle
            .push(population.get(population.len() / 2).unwrap().aptitude);
        stats.worst.push(population.last().unwrap().aptitude);

        for i in 0..population.len() {
            let mut values = merge(
                &population.get(i).unwrap().values,
                &population.get(population.len() - i - 1).unwrap().values,
            );
            mutate(&mut values, &constraints);
            let aptitude = evaluate(&values);
            population.remove(population.len() - i - 1);
            population.push(Individual { values, aptitude });
            current_evaluations += 1;
        }
    }

    stats
}

fn merge(father1: &[f64], father2: &[f64]) -> Vec<f64> {
    const FIRST_FATHER_CHANCE: f32 = 0.7;
    let mut child = Vec::new();

    for i in 0..father1.len() {
        child.push(if random_range(0.0..=1.0) <= FIRST_FATHER_CHANCE {
            father1[i]
        } else {
            father2[i]
        });
    }

    child
}

fn mutate(individual: &mut [f64], constraints: &HashMap<usize, RangeInclusive<f64>>) {
    let rand_index = random_range(0..5);
    individual[rand_index] = random_range(constraints.get(&rand_index).unwrap().clone());
}
