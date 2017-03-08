extern crate rand;
use rand::distributions::{IndependentSample, Range};

#[derive(Clone, Copy)]
struct Point {
    x1:f64,
    x2:f64,
}

fn objective_function(p:Point) -> f64 {
    //return p.x1.powf(2.) + p.x2.powf(2.); // Sphere
    return (1.5 - p.x1 + p.x1 * p.x2).powf(2.) + (2.25 - p.x1 + p.x1 * p.x2.powf(2.)).powf(2.) + (2.625 - p.x1 + p.x1 * p.x2.powf(3.)).powf(2.); // Beale's
    //return (p.x1 + 2.*p.x2 - 7.).powf(2.) + (2.*p.x1 + p.x2 - 5.).powf(2.); //  Booth's
}

fn main () {
    let initial_temperature:f64 = 1000.0;
    let final_temperature:f64 = 0.0;

    let iterations:i32 = 1_000_000;

    let box_size = 1.0;
    let random_limit = 5.0;

    let between = Range::new( -random_limit, random_limit);
    let mut rng = rand::thread_rng();

    let mut solution:Point = Point{x1:(box_size * (between.ind_sample(&mut rng) - 0.5)), x2: (box_size * (between.ind_sample(&mut rng) - 0.5))};
    let mut best:Point = solution;
    let mut best_energy = objective_function(best);

    println!("Initial Temperature: {0}", initial_temperature);
    println!("Final Temperature: {0}", final_temperature);
    println!("Running for {0} iterations", iterations);

    for iter in 0..iterations {
        let temp = initial_temperature - (iter as f64) * ((initial_temperature - final_temperature) as f64) / (iterations as f64);

        let candidate:Point = Point{x1:(solution.x1 + between.ind_sample(&mut rng)), x2: (solution.x2 + between.ind_sample(&mut rng) )};

        let current_energy = objective_function(solution);
        let candidate_energy = objective_function(candidate);
        let delta_t = candidate_energy - current_energy;

        if candidate_energy < best_energy {
            best_energy = candidate_energy;
            best = candidate;
            println!("NEW BEST: f({0:3.16} {1:3.16}) = {2:3.16}", best.x1, best.x2, objective_function(best));
        }

        if candidate_energy < current_energy {
            solution = candidate;
        } else if (-delta_t / temp).exp() > between.ind_sample(&mut rng) {
            solution = candidate;
        }

        //println!("f({0} {1}) = {2}", solution.x1, solution.x2, objective_function(solution));
        //println!("{3} {0} {1} {2}", solution.x1, solution.x2, objective_function(solution), iter);
    }

    println!("f({0} {1}) = {2}", best.x1, best.x2, objective_function(best));
}

