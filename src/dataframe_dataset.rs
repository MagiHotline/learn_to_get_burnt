use crate::player::Player;
use crate::utils::get_db_if_missing;
use burn_dataset::{DataframeDataset, Dataset};
use polars::prelude::*;

/// Players dataset using Polars dataframeDataset as the backend
pub struct PlayerDataframeDataset {
    dataset: DataframeDataset<Player>,
}

impl PlayerDataframeDataset {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let path = get_db_if_missing();

        // Cols definitions => (name, DataType)
        const COLS: &[(&str, DataType)] = &[
            ("player_id", DataType::UInt32),
            ("first_name", DataType::String),
            ("last_name", DataType::String),
            ("name", DataType::String),
            ("last_season", DataType::UInt32),
            ("current_club_id", DataType::UInt32),
            ("player_code", DataType::String),
            ("country_of_birth", DataType::String),
            ("city_of_birth", DataType::String),
            ("country_of_citizenship", DataType::String),
            ("date_of_birth", DataType::String),
            ("sub_position", DataType::String),
            ("position", DataType::String),
            ("foot", DataType::String),
            ("height_in_cm", DataType::UInt32),
            ("contract_expiration_date", DataType::String),
            ("agent_name", DataType::String),
            ("image_url", DataType::String),
            ("international_caps", DataType::UInt32),
            ("international_goals", DataType::UInt32),
            ("current_national_team_id", DataType::UInt32),
            ("url", DataType::String),
            ("current_club_domestic_competition_id", DataType::String),
            ("current_club_name", DataType::String),
            ("market_value_in_eur", DataType::UInt32),
            ("highest_market_value_in_eur", DataType::UInt32),
        ];

        let schema = Schema::from_iter(
            COLS.iter()
                .map(|(name, dtype)| Field::new((*name).into(), dtype.clone())),
        );

        let df = LazyCsvReader::new(PlRefPath::new(path.to_str().unwrap()))
            .with_has_header(true)
            .with_separator(b',')
            .with_schema(Some(Arc::new(schema)))
            .finish()?
            .collect()?;

        let dataset = DataframeDataset::new(df)?;

        Ok(Self { dataset })
    }
}

impl Default for PlayerDataframeDataset {
    fn default() -> Self {
        Self::new().expect("Could not load Player Transfermarkt dataset")
    }
}

impl Dataset<Player> for PlayerDataframeDataset {
    fn get(&self, index: usize) -> Option<Player> {
        self.dataset.get(index)
    }

    fn len(&self) -> usize {
        self.dataset.len()
    }
}
