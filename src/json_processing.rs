use std::{
    fs,
    io::{self, Read, Write}
};

pub const FILE_NAME: &'static str = "products.json";

pub fn read_file(file_name: &str) -> io::Result<fs::File> {
    fs::File::open(file_name).or_else(|_| {
        let mut f = fs::File::create(file_name).expect("Could not create a json file");
        f.write_all(b"[]").expect("Could not populate json file");

        Ok(f)
    })
}

pub fn open_json() -> serde_json::Result<Vec<serde_json::Value>> {
    let file = read_file(FILE_NAME).expect("Could not read a json file");

    let mut buf_reader = io::BufReader::new(&file);
    let mut contents = String::new();

    buf_reader.read_to_string(&mut contents).expect("Could not process a json file");

    let a: Vec<serde_json::Value> = serde_json::from_str(contents.as_str())?;

    Ok(a)
}

pub fn save_json(file_name: &str, json: serde_json::Value) -> io::Result<()> {
    let mut file = fs::File::create(file_name).expect("Could not create a file");

    file.write_all(json.to_string().as_bytes())?;
    Ok(())
}
