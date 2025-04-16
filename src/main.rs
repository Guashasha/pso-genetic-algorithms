use rand::random_range;

mod genetic;
mod plotter;
mod pso;

const POPULATION_SIZE: usize = 500;

fn main() {
    let mut pso_population: Vec<Vec<f64>> = generate_initial_population();
    let mut genetic_population: Vec<Vec<f64>> = generate_initial_population();
}

fn evaluate(individual: &[f64]) -> f64 {
    (5.3578547 * (f64::powi(individual[2], 2)))
        + (0.8356891 * individual[0] * individual[4])
        + (37.293239 * individual[0])
        - 40792.141
}

fn valid_inputs(individual: &[f64]) -> bool {
    if individual[0] < 78.0 || individual[0] > 102.0 || individual[1] < 33.0 || individual[1] > 45.0
    {
        return false;
    }

    for i in 2..5 {
        if individual[i] < 27.0 || individual[i] > 45.0 {
            return false;
        }
    }

    // g1
    if 85.334407
        + (0.0056858 * individual[1] * individual[4])
        + (0.0006262 * individual[0] * individual[3])
        - (0.0022053 * individual[2] * individual[4])
        > 0.0
    {
        return false;
    }

    // g2
    if -85.334407
        - (0.0056858 * individual[1] * individual[4])
        - (0.0006262 * individual[0] * individual[3])
        + (0.0022053 * individual[2] * individual[4])
        > 0.0
    {
        return false;
    }

    // g3
    if 80.51249
        + (0.0071317 * individual[1] * individual[4])
        + (0.0029955 * individual[0] * individual[1])
        + (0.0021813 * f64::powi(individual[2], 2))
        - 110.0
        > 0.0
    {
        return false;
    }

    // g4
    if -80.51249
        - (0.0071317 * individual[1] * individual[4])
        - (0.0029955 * individual[0] * individual[1])
        - (0.0021813 * f64::powi(individual[2], 2))
        + 90.0
        > 0.0
    {
        return false;
    }

    // g5
    if 9.300961
        + (0.0047026 * individual[2] * individual[4])
        + (0.0012547 * individual[0] * individual[2])
        + (0.0019085 * individual[2] * individual[3])
        - 25.0
        > 0.0
    {
        return false;
    }

    // g6
    if -9.300961
        - (0.0047026 * individual[2] * individual[4])
        - (0.0012547 * individual[0] * individual[2])
        - (0.0019085 * individual[2] * individual[3])
        + 20.0
        > 0.0
    {
        return false;
    }

    true
}

fn generate_initial_population() -> Vec<Vec<f64>> {
    let mut population = Vec::new();

    for _ in 0..POPULATION_SIZE {
        population.push(generate_individual());
    }

    population
}

fn generate_individual() -> Vec<f64> {
    vec![
        random_range(78.0..103.0),
        random_range(33.0..46.0),
        random_range(27.0..46.0),
        random_range(27.0..46.0),
        random_range(27.0..46.0),
    ]
}
