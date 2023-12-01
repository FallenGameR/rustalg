// Experiment 1.1.35 Dice simulation.
// Calculate the theoretical probability of each sum of two dices.
// Compare with the random experiment.
// How many dice throws are needed to get an agreement with precision better than 0.001?
//
// Answer: 100k-200k throws are needed

use rand::Rng;

const DICE: usize = 6;
const THRESHOLD: f64 = 0.001;

fn get_theoretical_frequency() -> Vec<usize> {
    let mut frequency = vec![0; DICE * 2 + 1];
    for first in 1..=DICE {
        for second in 1..=DICE {
            frequency[first + second] += 1;
        }
    }
    frequency
}

fn get_probability(frequency: &Vec<usize>, n: usize) -> Vec<f64> {
    let mut probability = vec![0.0; DICE * 2 + 1];
    for i in 2..=DICE * 2 {
        probability[i] = frequency[i] as f64 / n as f64;
    }
    probability
}

fn main() {
    let theoretical_frequency = get_theoretical_frequency();
    let theoretical_probability = get_probability(&theoretical_frequency, DICE * DICE);
    dbg!(&theoretical_probability);

    let mut frequency = vec![0; DICE * 2 + 1];
    let mut rng = rand::thread_rng();
    let mut n = 0;

    loop {
        n += 1;
        let first: usize = rng.gen_range(1..=6);
        let second: usize = rng.gen_range(1..=6);
        frequency[first + second] += 1;

        let probability = frequency[first + second] as f64 / n as f64;
        let diff = (probability - theoretical_probability[first + second]).abs();
        if diff < THRESHOLD {
            let probabilities = get_probability(&frequency, n);
            let mut all_bellow = true;

            for i in 2..=DICE * 2 {
                let diff = (probabilities[i] - theoretical_probability[i]).abs();
                if diff >= THRESHOLD {
                    all_bellow = false;
                    break;
                }
            }

            if all_bellow {
                println!("n = {}", n);
                break;
            }
        }
    }

    let probabilities = get_probability(&frequency, n);
    dbg!(probabilities);
}
