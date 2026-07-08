use burn::data::dataset::{Dataset, InMemDataset};

use crate::player::Player;
use crate::utils::get_db_if_missing;

pub struct PlayersDataset {
    dataset: InMemDataset<Player>,
}

impl PlayersDataset {
    pub fn new() -> Result<Self, std::io::Error> {
        let path = get_db_if_missing();

        let mut reader = csv::ReaderBuilder::new();
        let reader = reader.delimiter(b',');

        let dataset = InMemDataset::from_csv(path, reader).unwrap();

        Ok(Self { dataset })
    }
}

impl Dataset<Player> for PlayersDataset {
    fn get(&self, index: usize) -> Option<Player> {
        self.dataset.get(index)
    }

    fn len(&self) -> usize {
        self.dataset.len()
    }
}
