// Experiment 1.1.35 Dice simulation.
// Calculate the theoretical probability of each sum of two dices.
// Compare with the random experiment.
// How many dice throws are needed to get an agreement with precision better than 0.001?

const DICE: usize = 6;

fn get_frequency() -> Vec<usize> {
    let mut frequency = vec![0; DICE * 2 + 1];
    for first in 1..=DICE {
        for second in 1..=DICE {
            frequency[first + second] += 1;
        }
    }
    frequency
}

fn get_probability(frequency: &Vec<usize>) -> Vec<f64> {
    let total = (DICE * DICE) as f64;
    let mut probability = vec![0.0; DICE * 2 + 1];
    for i in 2..=DICE * 2 {
        probability[i] = frequency[i] as f64 / total;
    }
    probability
}

fn main() {
    let frequency = get_frequency();
    let probability = get_probability(&frequency);

    dbg!(frequency);
    dbg!(probability);
}
