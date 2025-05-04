use rand::seq::IndexedRandom;
use std::{
    fs::File,
    io::{Read, Write},
};

const DB_FILENAME: &str = "seen_episodes.txt";

pub fn get_next_episode() -> Result<&'static str, std::io::Error> {
    let mut seen_episodes = read_db_from_file(DB_FILENAME)?;

    let selected_episode = select_next_episode(&seen_episodes);

    seen_episodes.push(selected_episode.to_string());
    save_db_to_file(seen_episodes, DB_FILENAME)?;

    Ok(selected_episode)
}

fn read_db_from_file(db_filename: &str) -> Result<Vec<String>, std::io::Error> {
    let mut file = match File::open(db_filename) {
        Ok(file) => file,
        Err(err) => match err.kind() {
            std::io::ErrorKind::NotFound => return Ok(Vec::new()),
            _ => return Err(err),
        },
    };

    let mut seen_episodes = String::new();
    file.read_to_string(&mut seen_episodes)?;

    Ok(seen_episodes
        .trim()
        .lines()
        .rev()
        .map(|s| s.to_string())
        .collect())
}

fn select_next_episode(seen_episodes: &[String]) -> &'static str {
    let seen_set: std::collections::HashSet<&str> = seen_episodes
        .iter()
        .map(|s| s.as_str())
        .collect();

    EPISODES
        .choose_multiple(&mut rand::rng(), EPISODES.len())
        .find(|ep| !seen_set.contains(*ep))
        .expect("No unseen episodes available")
}

fn save_db_to_file(seen_episodes: Vec<String>, db_filename: &str) -> Result<(), std::io::Error> {
    if seen_episodes.is_empty() {
        return Ok(());
    }

    let mut file = File::create(db_filename)?;

    write!(
        file,
        "{}",
        seen_episodes
            .iter()
            .fold(String::new(), |acc, s| format!("{}\n{}", s, acc))
    )?;

    Ok(())
}

#[cfg(test)]
mod test {
    use tempfile::NamedTempFile;

    use super::*;

    #[test]
    fn read_db_from_file_works_with_non_existing_file() {
        let result = read_db_from_file("non_existing_file.txt");
        assert!(!result.is_err(), "result is error: {result:#?}");
        assert_eq!(result.unwrap(), Vec::<String>::new());
    }

    #[test]
    fn read_db_from_file_handles_empty_file() {
        let tmpfile = NamedTempFile::new().unwrap();

        let result = read_db_from_file(
            &tmpfile
                .path()
                .to_string_lossy()
                .into_owned(),
        );
        assert!(!result.is_err(), "result is error: {result:#?}");
        assert_eq!(result.unwrap(), Vec::<String>::new());
    }

    #[test]
    fn read_db_from_file_reads_data_from_file() {
        let mut tmpfile = NamedTempFile::new().unwrap();
        writeln!(tmpfile, "s01e02\ns01e01\n").unwrap();

        let result = read_db_from_file(
            &tmpfile
                .path()
                .to_string_lossy()
                .into_owned(),
        );
        assert!(!result.is_err(), "result is error: {result:#?}");
        assert_eq!(result.unwrap(), vec!("s01e01", "s01e02"));
    }

    #[test]
    fn select_next_episode_returns_any_episode_at_all() {
        let result = select_next_episode(&Vec::new());
        assert!(EPISODES.contains(&result))
    }

    #[test]
    #[should_panic(expected = "No unseen episodes available")]
    fn select_next_episode_panics_if_there_is_no_unseen_episodes() {
        let all_episodes: Vec<String> = EPISODES
            .iter()
            .map(|s| s.to_string())
            .collect();
        select_next_episode(&all_episodes);
    }

    #[test]
    fn save_db_to_file_saves_empty_list_to_file() {
        let mut tmpfile = NamedTempFile::new().unwrap();

        let result = save_db_to_file(
            Vec::new(),
            &tmpfile
                .path()
                .to_string_lossy()
                .into_owned(),
        );
        assert!(!result.is_err(), "result is error: {result:#?}");

        let mut tmpfile_content = String::new();
        tmpfile
            .read_to_string(&mut tmpfile_content)
            .unwrap();
        assert_eq!(tmpfile_content, "");
    }

    #[test]
    fn save_db_to_file_saves_non_empty_list_to_file_in_reverse_order() {
        let mut tmpfile = NamedTempFile::new().unwrap();

        let seen_episodes = vec!["s01e01", "s01e02"]
            .iter()
            .map(|s| s.to_string())
            .collect();

        let result = save_db_to_file(
            seen_episodes,
            &tmpfile
                .path()
                .to_string_lossy()
                .into_owned(),
        );
        assert!(!result.is_err(), "result is error: {result:#?}");

        let mut tmpfile_content = String::new();
        tmpfile
            .read_to_string(&mut tmpfile_content)
            .unwrap();
        assert_eq!(tmpfile_content, "s01e02\ns01e01\n");
    }
}

const EPISODES: [&str; 234] = [
    "s01e01", "s01e02", "s01e03", "s01e04", "s01e05", "s01e06", "s01e07", "s01e08", "s01e09",
    "s01e10", "s01e11", "s01e12", "s01e13", "s01e14", "s01e15", "s01e16", "s01e17", "s01e18",
    "s01e19", "s01e20", "s01e21", "s01e22", "s01e23", "s01e24", "s02e01", "s02e02", "s02e03",
    "s02e04", "s02e05", "s02e06", "s02e07", "s02e08", "s02e09", "s02e10", "s02e11", "s02e12",
    "s02e13", "s02e14", "s02e15", "s02e16", "s02e17", "s02e18", "s02e19", "s02e20", "s02e21",
    "s02e22", "s02e23", "s02e24", "s03e01", "s03e02", "s03e03", "s03e04", "s03e05", "s03e06",
    "s03e07", "s03e08", "s03e09", "s03e10", "s03e11", "s03e12", "s03e13", "s03e14", "s03e15",
    "s03e16", "s03e17", "s03e18", "s03e19", "s03e20", "s03e21", "s03e22", "s03e23", "s03e24",
    "s03e25", "s04e01", "s04e02", "s04e03", "s04e04", "s04e05", "s04e06", "s04e07", "s04e08",
    "s04e09", "s04e10", "s04e11", "s04e12", "s04e13", "s04e14", "s04e15", "s04e16", "s04e17",
    "s04e18", "s04e19", "s04e20", "s04e21", "s04e22", "s04e23", "s04e24", "s05e01", "s05e02",
    "s05e03", "s05e04", "s05e05", "s05e06", "s05e07", "s05e08", "s05e09", "s05e10", "s05e11",
    "s05e12", "s05e13", "s05e14", "s05e15", "s05e16", "s05e17", "s05e18", "s05e19", "s05e20",
    "s05e21", "s05e22", "s05e23", "s05e24", "s06e01", "s06e02", "s06e03", "s06e04", "s06e05",
    "s06e06", "s06e07", "s06e08", "s06e09", "s06e10", "s06e11", "s06e12", "s06e13", "s06e14",
    "s06e15", "s06e16", "s06e17", "s06e18", "s06e19", "s06e20", "s06e21", "s06e22", "s06e23",
    "s06e24", "s06e25", "s07e01", "s07e02", "s07e03", "s07e04", "s07e05", "s07e06", "s07e07",
    "s07e08", "s07e09", "s07e10", "s07e11", "s07e12", "s07e13", "s07e14", "s07e15", "s07e16",
    "s07e17", "s07e18", "s07e19", "s07e20", "s07e21", "s07e22", "s07e23", "s07e24", "s08e01",
    "s08e02", "s08e03", "s08e04", "s08e05", "s08e06", "s08e07", "s08e08", "s08e09", "s08e10",
    "s08e11", "s08e12", "s08e13", "s08e14", "s08e15", "s08e16", "s08e17", "s08e18", "s08e19",
    "s08e20", "s08e21", "s08e22", "s08e23", "s08e24", "s09e01", "s09e02", "s09e03", "s09e04",
    "s09e05", "s09e06", "s09e07", "s09e08", "s09e09", "s09e10", "s09e11", "s09e12", "s09e13",
    "s09e14", "s09e15", "s09e16", "s09e17", "s09e18", "s09e19", "s09e20", "s09e21", "s09e22",
    "s09e23", "s10e01", "s10e02", "s10e03", "s10e04", "s10e05", "s10e06", "s10e07", "s10e08",
    "s10e09", "s10e10", "s10e11", "s10e12", "s10e13", "s10e14", "s10e15", "s10e16", "s10e17",
];
