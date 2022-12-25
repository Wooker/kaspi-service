use uuid::Uuid;
use tokio::sync::Mutex;
use std::collections::HashMap;
use serde_json::json;
use crate::{
    entities::product::{Product, Record},
    entities::upload_result::{Status, UploadResult},
    json_processing::{read_json, FILE_NAME},
};


pub struct Store {
    results: Mutex<HashMap<Uuid, UploadResult>>,
    products: Mutex<HashMap<Uuid, Product>>,
    uploaded: Mutex<HashMap<Uuid, String>>,
    finished: Mutex<HashMap<Uuid, String>>,
    aborted: Mutex<HashMap<Uuid, String>>
}

impl Store {
    pub fn new() -> Self {
        Self {
            results: Mutex::new(HashMap::new()),
            products: Mutex::new(HashMap::new()),
            uploaded: Mutex::new(HashMap::new()),
            finished: Mutex::new(HashMap::new()),
            aborted: Mutex::new(HashMap::new()),
        }
    }

    pub async fn fill(&self) {
        let json = read_json(FILE_NAME).await.expect("Could not read json file");

        for entry in json.into_iter() {
            let id = Uuid::parse_str(entry["id"].as_str().unwrap()).unwrap();

            let code = entry["code"].as_str().unwrap().to_string();
            let has_result = !entry["result"].is_null();

            match Status::from(entry["status"].as_str().unwrap()) {
                Status::UPLOADED => self.uploaded.lock().await.insert(id, code),
                Status::FINISHED => self.finished.lock().await.insert(id, code),
                Status::ABORTED => self.aborted.lock().await.insert(id, code),
            };

            let product: Product = serde_json::from_value(entry["product"].clone()).unwrap();
            self.products.lock().await.insert(id, product);

            if has_result {
                let result: UploadResult = serde_json::from_value(entry["result"].clone()).unwrap();
                self.results.lock().await.insert(id, result);
            }
        }
    }

    /// Returns None if the code was not present
    pub async fn insert_upload(&self, id: Uuid, code: String) -> Option<String> {
        self.uploaded
            .lock()
            .await
            .insert(id, code)
    }

    /// Returns None if the product was not present
    pub async fn insert_product(&self, id: Uuid, product: Product) -> Option<Product> {
        self.products
            .lock()
            .await
            .insert(id, product)
    }

    /// Returns None if the product was not present
    pub async fn insert_result(&self, id: Uuid, result: UploadResult) -> Option<UploadResult> {
        self.results
            .lock()
            .await
            .insert(id, result)
    }

    /// Returns the code and the status of uploading
    /// Otherwise, _None_
    pub async fn get_status(&self, id: &Uuid) -> Option<(String, Status)> {
        if let Some(code) = self.uploaded.lock().await.get(id) {
            Some((code.clone(), Status::UPLOADED))
        } else if let Some(code) = self.finished.lock().await.get(id) {
            Some((code.clone(), Status::FINISHED))
        } else if let Some(code) = self.aborted.lock().await.get(id) {
            Some((code.clone(), Status::ABORTED))
        } else {
            None
        }
    }
    /// Returns the product
    /// Otherwise, _None_
    pub async fn get_product(&self, id: &Uuid) -> Option<Product> {
        self.products.lock().await.get(id).cloned()
    }
    /// Returns the result of uploading
    /// Otherwise, _None_
    pub async fn get_result(&self, id: &Uuid) -> Option<UploadResult> {
        self.results.lock().await.get(id).cloned()
    }

    /// Returns the code of uploading, if the id is moved successfuly
    pub async fn archive(&self, id: &Uuid, status: Status) -> Option<String> {
        let code = self.uploaded
            .lock()
            .await
            .remove(id)
            .expect("Could not remove from the store");

        let mut added: Option<String> = None;
        match status {
            Status::FINISHED => added = self.finished.lock().await.insert(id.to_owned(), code),
            Status::ABORTED => added = self.aborted.lock().await.insert(id.to_owned(), code),
            _ => {}
        }

        added
    }

    pub async fn uploaded_len(&self) -> usize {
        self.uploaded.lock().await.len()
    }

    pub async fn finished_len(&self) -> usize {
        self.finished.lock().await.len()
    }

    pub async fn aborted_len(&self) -> usize {
        self.aborted.lock().await.len()
    }

    pub async fn uploaded_ids(&self) -> Vec<Uuid> {
        self.uploaded.lock().await.keys().cloned().collect()
    }

    pub async fn products(&self) -> tokio::sync::MutexGuard<'_, HashMap<Uuid, Product>> {
        self.products.lock().await
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[actix_rt::test]
    async fn insert_get_remove() {
        let store = Store::new();

        let code = String::from("0000001");
        let id = Uuid::new_v5(&uuid::Uuid::NAMESPACE_URL, code.as_bytes());

        store.insert_upload(id, code.clone()).await;

        let other_code = String::from("0000001");
        let other_id = Uuid::new_v5(&uuid::Uuid::NAMESPACE_URL, code.as_bytes());

        assert!(store.insert_upload(other_id, other_code).await.is_some());

        assert_eq!(store.get_status(id).await.unwrap(), (code, Status::UPLOADED));
        assert_eq!(store.uploaded_len().await, 1);

        store.archive(id, Status::FINISHED).await;
        assert_eq!(store.uploaded_len().await, 0);
    }
}
