use rand::distributions::{Distribution, Uniform};
use rand::Rng;
// cerner_2tothe5th_2021

struct GameResults {
    win: bool,1
    switch: bool,
}

fn play_game<R: Rng>(random_door: &Uniform<u32>, rng: &mut R) -> GameResults {
    let car = random_door.sample(rng);

    let mut choice = random_door.sample(rng);


    let open = monty_open(car, choice, rng);


    let switch = rng.gen();
    if switch {
        choice = switch_door(choice, open);
    }

    GameResults {
        win: choice == car,
        switch,
    }
}

fn monty_open<R: Rng>(car: u32, choice: u32, rng: &mut R) -> u32 {
    use rand::seq::SliceRandom;
    *free_doors(&[car, choice]).choose(rng).unwrap()
}

fn free_doors(blocked: &[u32]) -> Vec<u32> {
    (1..=3).filter(|x| !blocked.contains(x)).collect()
}

// Returns the door we switch to, given our current choice and
// the open door. There will only be one valid door.
fn switch_door(choice: u32, open: u32) -> u32 {
    free_doors(&[choice, open])[0]
}

fn main() {
    let num_games = 1000;
    let mut rng = rand::thread_rng();

    let random_door = Uniform::new(0u32, 3);

    let (mut switch_wins, mut switch_losses) = (0, 0);
    let (mut keep_wins, mut keep_losses) = (0, 0);

    println!("Running {} simulations...", num_games);
    for _ in 0..num_games {
        let result = play_game(&random_door, &mut rng);

        match (result.win, result.switch) {
            (true, true) => switch_wins += 1,
            (true, false) => keep_wins += 1,
            (false, true) => switch_losses += 1,
            (false, false) => keep_losses += 1,
        }
    }

    let total_switches = switch_wins + switch_losses;
    let total_keeps = keep_wins + keep_losses;

    println!(
        "Switched door {} times with {} wins and {} losses",
        total_switches, switch_wins, switch_losses
    );

    println!(
        "Kept our choice {} times with {} wins and {} losses",
        total_keeps, keep_wins, keep_losses
    );

    // With a large number of simulations, the values should converge to
    // 0.667 and 0.333 respectively.
    println!(
        "Estimated chance to win if we switch: {}",
        switch_wins as f32 / total_switches as f32
    );
    println!(
        "Estimated chance to win if we don't: {}",
        keep_wins as f32 / total_keeps as f32
    );
}
