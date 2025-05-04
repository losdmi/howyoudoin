use std::collections::HashMap;

fn main() {
    let next_episode = match howyoudoin::get_next_episode() {
        Err(err) => {
            eprintln!("{err}");
            return;
        }
        Ok(next_episode) => next_episode,
    };

    println!("{next_episode}");
    println!();

    println!("Press Enter to exit...");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Should never error");
}

fn _generate_episodes_list() {
    let mut seasons_with_episodes = HashMap::from([
        (1, 1..=24),
        (2, 1..=24),
        (3, 1..=25),
        (4, 1..=24),
        (5, 1..=24),
        (6, 1..=25),
        (7, 1..=24),
        (8, 1..=24),
        (9, 1..=23),
        (10, 1..=17),
    ]);

    for season in 1..=10 {
        if let Some(episodes) = seasons_with_episodes.remove(&season) {
            for episode in episodes {
                println!("\"s{season:02}e{episode:02}\",");
            }
        }
    }
}
