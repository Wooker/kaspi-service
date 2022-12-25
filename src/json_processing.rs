use crate::entities::product::Record;
use tokio::{fs, io::{self, AsyncReadExt, AsyncWriteExt}};

pub const FILE_NAME: &'static str = "products.json";

pub async fn open_file(file_name: &str) -> io::Result<fs::File> {
    let res = fs::File::open(file_name).await;

    match res {
        Ok(f) => Ok(f),
        Err(_) => {
            let mut f = fs::File::create(file_name).await.expect("Could not create a json file");
            f.write_all(b"[]").await.expect("Could not populate json file");

            Ok(f)
        }
    }
}

pub async fn read_json(file_name: &str) -> serde_json::Result<Vec<serde_json::Value>> {
    let file = open_file(file_name).await.expect("Could not read a json file");

    let mut buf_reader = io::BufReader::new(file);
    let mut contents = String::new();

    buf_reader.read_to_string(&mut contents).await.expect("Could not process a json file");

    let a: Vec<serde_json::Value> = serde_json::from_str(contents.as_str())?;

    Ok(a)
}

pub async fn save_json(file_name: &str, json: serde_json::Value) -> io::Result<()> {
    let mut file = fs::File::create(file_name).await.expect("Could not create a file");

    file.write_all(json.to_string().as_bytes()).await?;
    Ok(())
}

pub async fn modify_record(mut func: impl FnMut(Vec<Record>) -> anyhow::Result<Vec<Record>>) -> anyhow::Result<()> {
    let mut file = open_file(FILE_NAME).await.expect("Could not read a json file");

    let mut buf_reader = io::BufReader::new(&mut file);
    let mut contents = String::new();

    buf_reader.read_to_string(&mut contents).await.expect("Could not process a json file");

    let vec = serde_json::to_value(contents.as_str()).expect("Could not create json");
    let mut records: Vec<Record> = serde_json::from_value(vec)?;

    records = func(records)?;

    file.write_all(
        serde_json::to_value(records)
        .expect("Could not create json")
        .to_string()
        .as_bytes()
    ).await?;

    Ok(())
}
