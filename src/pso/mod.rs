use crate::{evaluate, valid_inputs, EvolutionStats, MAX_EVUALUATIONS, POPULATION_SIZE};

const GLOBAL_BEST_ATRACTION: f64 = 0.8;
const PERSONAL_BEST_ATRACTION: f64 = 0.5;
const MOVEMENT_VARIATION: f64 = 2.0;
const INERTIA: f64 = 0.6;

struct Particle {
    current: Vec<f64>,
    current_aptitude: f64,
    best: Vec<f64>,
    best_aptitude: f64,
    velocity: Vec<f64>,
}

struct Swarm {
    population: Vec<Particle>,
    best: Vec<f64>,
    best_aptitude: f64,
}

pub fn evolve(population: Vec<Vec<f64>>) -> EvolutionStats {
    let mut swarm = Swarm::from(population);
    let mut stats = EvolutionStats::new();
    let mut current_evaluations = POPULATION_SIZE;

    while current_evaluations < MAX_EVUALUATIONS {
        swarm
            .population
            .sort_by(|a, b| a.current_aptitude.partial_cmp(&b.current_aptitude).unwrap());
        stats
            .best
            .push(swarm.population.first().unwrap().current_aptitude);
        stats
            .worst
            .push(swarm.population.last().unwrap().current_aptitude);
        stats.middle.push(
            swarm
                .population
                .get(POPULATION_SIZE / 2)
                .unwrap()
                .current_aptitude,
        );

        for i in 0..swarm.population.len() {
            current_evaluations += swarm.population[i].continue_movement(&swarm.best);
        }
    }

    stats
}

impl Swarm {
    fn from(population: Vec<Vec<f64>>) -> Self {
        let mut swarm_population = Vec::new();
        let mut swarm_best = Vec::new();
        let mut swarm_best_aptitude = f64::MAX;

        for individual in population {
            let best_aptitude = evaluate(&individual);

            swarm_population.push(Particle {
                current: individual.clone(),
                current_aptitude: best_aptitude,
                best: individual.clone(),
                best_aptitude,
                velocity: vec![0.0, 0.0, 0.0, 0.0, 0.0],
            });

            if best_aptitude < swarm_best_aptitude {
                swarm_best = individual;
                swarm_best_aptitude = best_aptitude;
            }
        }

        Self {
            population: swarm_population,
            best: swarm_best,
            best_aptitude: swarm_best_aptitude,
        }
    }
}

impl Particle {
    fn continue_movement(&mut self, global_best: &[f64]) -> usize {
        let mut count = 0;

        loop {
            let v1 = mult_vector(&self.velocity, INERTIA);
            let v2 = mult_vector(
                &sub_vectors(&self.best, &self.current),
                MOVEMENT_VARIATION * PERSONAL_BEST_ATRACTION,
            );
            let v3 = mult_vector(
                &sub_vectors(&global_best, &self.current),
                MOVEMENT_VARIATION * GLOBAL_BEST_ATRACTION,
            );
            let v4 = sum_vectors(&v1, &v2);
            let new_velocity = sum_vectors(&v3, &v4);
            let mut new_position = sum_vectors(&new_velocity, &self.current);
            Self::comply_with_limits(&mut new_position);

            count += 1;
            if valid_inputs(&self.current) {
                let new_aptitude = evaluate(&new_position);
                if self.best_aptitude > new_aptitude {
                    self.best_aptitude = new_aptitude;
                    self.best = new_position.clone();
                }

                self.current_aptitude = new_aptitude;
                self.current = new_position;
                return count;
            }

            println!("valores invalidos, continuando movimiento");
        }
    }

    fn comply_with_limits(position: &mut [f64]) {
        match position[0] {
            val if val > 102.0 => position[0] = 102.0,
            val if val < 78.0 => position[0] = 78.0,
            _ => (),
        }

        match position[1] {
            val if val > 45.0 => position[1] = 45.0,
            val if val < 33.0 => position[1] = 33.0,
            _ => (),
        }

        for i in 2..5 {
            match position[i] {
                val if val < 27.0 => position[i] = 27.0,
                val if val > 45.0 => position[i] = 45.0,
                _ => (),
            }
        }
    }
}

fn sum_vectors(v1: &[f64], v2: &[f64]) -> Vec<f64> {
    let mut new_vector = Vec::new();

    for i in 0..v1.len() {
        new_vector.push(v1[i] + v2[i]);
    }

    new_vector
}

fn sub_vectors(v1: &[f64], v2: &[f64]) -> Vec<f64> {
    let mut new_vector = Vec::new();

    for i in 0..v1.len() {
        new_vector.push(v1[i] - v2[i]);
    }

    new_vector
}

fn mult_vector(v1: &[f64], n: f64) -> Vec<f64> {
    v1.iter().map(|item| item * n).collect()
}
