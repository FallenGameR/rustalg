use rand::Rng;

const DICE: usize = 6;
const THRESHOLD: f64 = 0.001;

///
/// # Experiment `1.1.35` - Dice simulation
///
/// Calculate the theoretical probability of each sum of
/// two d6 `DICE` dices and compare with the a random experiment.
///
/// How many dice throws are needed to get an agreement
/// with precision `THRESHOLD` better than 0.001?
///
/// Answer: 50k-400k throws are needed, depends on the random seed.
///
fn main() {
    let theoretical_frequency = get_theoretical_frequency();
    let theoretical_probability = get_probability(&theoretical_frequency, DICE * DICE);
    dbg!(&theoretical_probability);

    let mut frequency = vec![0; DICE + DICE + 1];
    let mut rng = rand::rng();
    let mut n = 0;

    loop {
        n += 1;
        let first: usize = rng.random_range(1..=DICE);
        let second: usize = rng.random_range(1..=DICE);
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

fn get_theoretical_frequency() -> Vec<usize> {
    // Elements at 0,1 are unused, as dice sums start from 2
    // That allows to use index as the sum of two dice we want to get the frequency for
    let mut frequency = vec![0; DICE + DICE + 1];

    for first in 1..=DICE {
        for second in 1..=DICE {
            frequency[first + second] += 1;
        }
    }

    frequency
}

fn get_probability(frequency: &Vec<usize>, n: usize) -> Vec<f64> {
    let mut probability = vec![0.0; DICE + DICE + 1];

    for i in 2..=DICE * 2 {
        probability[i] = frequency[i] as f64 / n as f64;
    }

    probability
}