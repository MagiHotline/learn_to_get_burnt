use burn_dataset::Dataset;
use custom_csv_dataset::dataset::PlayersDataset;

fn main() {
    let dataset = PlayersDataset::new().expect("Could not load players dataset");

    println!("Dataset loaded with {} rows", dataset.len());

    // get first element
    let item = dataset.get(0).unwrap();
    println!("First item: {:?}", item);
}
