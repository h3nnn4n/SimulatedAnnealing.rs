extern crate rand;
use rand::Rng;
use rand::distributions::{IndependentSample, Range};

#[derive(Clone, Copy)]
struct Point {
    x1:f64,
    x2:f64,
}

fn objective_function(p:Point) -> f64 {
    return p.x1.powf(2.) + p.x2.powf(2.);
}

fn main () {
    let initial_temperature:f64 = 100.0;
    let final_temperature:f64 = 0.0;

    let iterations:i32 = 1000;

    let mut solution:Point = Point{x1:0.2, x2:5.1};

    let between = Range::new(0., 1.);
    let mut rng = rand::thread_rng();

    println!("Initial Temperature: {0}", initial_temperature);
    println!("Final Temperature: {0}", final_temperature);
    println!("Running for {0} iterations", iterations);

    for iter in 0..iterations {
        let temp = initial_temperature - (iter as f64) * ((initial_temperature - final_temperature) as f64) / (iterations as f64);

        let candidate:Point = Point{x1:(solution.x1 + (between.ind_sample(&mut rng) - 0.5)), x2: (solution.x2 + (between.ind_sample(&mut rng) - 0.5))};

        let current_energy = objective_function(solution);
        let candidate_energy = objective_function(candidate);
        let delta_t = candidate_energy - current_energy;

        if candidate_energy > current_energy {
            solution = candidate;
        } else if (-delta_t / temp).exp() > between.ind_sample(&mut rng) {
            println!("WOW");
            solution = candidate;
        }

        println!("fx({0} {1}) = {2}", solution.x1, solution.x2, objective_function(solution));
    }
}

