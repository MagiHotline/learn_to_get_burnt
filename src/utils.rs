use std::{fs::File, path::{Path, PathBuf}, io::copy};

pub fn get_db_if_missing() -> PathBuf {

    let this_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let file_name = this_dir.join("database.csv");

    if file_name.exists() {
        println!("Database {file_name:?} already downloaded");
    } else {

        println!("Downloading {file_name:?} ...");
        let url = "https://pub-e682421888d945d684bcae8890b0ec20.r2.dev/data/players.csv.gz";

        let mut response = reqwest::blocking::get(url).unwrap();

        let mut file = File::create(&file_name).unwrap();
        copy(&mut response, &mut file).unwrap();
    }

    file_name
}
