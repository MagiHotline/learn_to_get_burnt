use burn_dataset::Dataset;
use custom_csv_dataset::dataframe_dataset::PlayerDataframeDataset;

fn main() {
    let dataset = PlayerDataframeDataset::new()
        .expect("Could not load players dataset with DataframeDatasets");

    println!(
        "Dataset loaded with {} rows using DataframeDatasets",
        dataset.len()
    );

    // get first element
    let item = dataset
        .get(0)
        .expect("Couldn't be able to get the first player");
    println!("First item: {:?}", item);
}
