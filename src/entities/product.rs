use serde::{Deserialize, Serialize};
use crate::entities::{
    attribute::Attribute,
    upload_result::{UploadStatus, UploadResult},
};
use std::fmt;
use uuid::Uuid;

/*
#[derive(Error, Debug)]
pub enum ProductError {
    #[error("Could not parse input")]
    Parsing(#[from] FromStr::Error),
    CategoryFetch,
    Images,
}
*/

#[derive(Serialize, Deserialize, Eq, Hash, PartialEq, Debug, Clone)]
struct ProductImage {
    url: String
}

#[derive(Serialize, Deserialize, Eq, Hash, PartialEq, Debug, Clone)]
pub struct Product {
    sku: String,
    title: String,
    brand: String,
    category: String,
    description: String,
    attributes: Vec<Attribute>,
    images: Vec<ProductImage>,
}

impl Product {
    pub fn sku(&self) -> &String {
        &self.sku
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq, Clone)]
pub struct Record {
    id: Uuid,
    code: String,
    product: Product,
    status: UploadStatus,
    result: Option<UploadResult>,
}

impl Record {
    pub fn sku(&self) -> &String {
        &self.product.sku
    }

    pub fn product(&self) -> &Product {
        &self.product
    }
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use crate::entities::{
        product::{Product, ProductImage},
        attribute::{Attribute, AttributeValue}, upload_result::UploadResult,
    };
    use serde_json::json;
    use uuid::Uuid;

    use super::Record;

    #[test]
    fn product_to_json() {
        let product = Product {
            sku: "LACEFRONT-27".to_string(),
            title: "Title".to_string(),
            brand: "ParikiAlmaty".to_string(),
            category: "Pariki".to_string(),
            description: "description".to_string(),
            attributes: vec![ Attribute {
                code: String::from("Wigs and hairpieces*Harakteristiki.wigs and hairpieces*purpose"),
                value: AttributeValue::String(String::from("zhenskiy"))
            }],
            images: vec![ ProductImage {
                url: "https://old.pariki.kz/wp-content/uploads/2022/11/photo_5452090876806414405_y.jpg".to_string()
            }],
        };

        let json = serde_json::to_value(product).unwrap();

        assert_eq!(
            json,
            serde_json::json!(
                {
                    "sku": "LACEFRONT-27",
                    "title": "Title",
                    "brand": "ParikiAlmaty",
                    "category": "Pariki",
                    "description": "description",
                    "attributes": [
                        {
                            "code": "Wigs and hairpieces*Harakteristiki.wigs and hairpieces*purpose",
                            "value": "zhenskiy"
                        }
                    ],
                    "images": [
                        {
                            "url": "https://old.pariki.kz/wp-content/uploads/2022/11/photo_5452090876806414405_y.jpg"
                        }
                    ]
                }
            )
        );
    }

    #[test]
    fn json_to_product() {
        let value = serde_json::json!(
            {
                "sku": "LACEFRONT-27",
                "title": "Title",
                "brand": "ParikiAlmaty",
                "category": "Pariki",
                "description": "description",
                "attributes": [
                    {
                        "code": "Wigs and hairpieces*Harakteristiki.wigs and hairpieces*purpose",
                        "value": "zhenskiy"
                    }
                ],
                "images": [
                    {
                        "url": "https://old.pariki.kz/wp-content/uploads/2022/11/photo_5452090876806414405_y.jpg"
                    }
                ]
            }
        );

        let product_from_json: Product = serde_json::from_value(value).unwrap();
        let product = Product {
            sku: "LACEFRONT-27".to_string(),
            title: "Title".to_string(),
            brand: "ParikiAlmaty".to_string(),
            category: "Pariki".to_string(),
            description: "description".to_string(),
            attributes: vec![ Attribute {
                code: String::from("Wigs and hairpieces*Harakteristiki.wigs and hairpieces*purpose"),
                value: AttributeValue::String(String::from("zhenskiy"))
            }],
            images: vec![ ProductImage {
                url: "https://old.pariki.kz/wp-content/uploads/2022/11/photo_5452090876806414405_y.jpg".to_string()
            }],
        };

        assert_eq!(product, product_from_json);
    }

    #[test]
    fn record_from_json() {
        let product = serde_json::json!(
            {
                "sku": "LACEFRONT-27",
                "title": "Title",
                "brand": "ParikiAlmaty",
                "category": "Pariki",
                "description": "description",
                "attributes": [
                    {
                        "code": "Wigs and hairpieces*Harakteristiki.wigs and hairpieces*purpose",
                        "value": "zhenskiy"
                    }
                ],
                "images": [
                    {
                        "url": "https://old.pariki.kz/wp-content/uploads/2022/11/photo_5452090876806414405_y.jpg"
                    }
                ]
            }
        );
        let id = Uuid::new_v5(&Uuid::NAMESPACE_URL, serde_json::to_string(&product).unwrap().as_bytes());

        let record_json = json!({
            "id": id,
            "product": product,
            "result": null,
            "code": "00101001"
        });

        let record: Record = serde_json::from_value(record_json.clone()).unwrap();
        assert_eq!(record_json, serde_json::to_value(record).unwrap());
    }

    #[test]
    fn upload_result() {
        let result_json = serde_json::json!(
            {
                "errors": 0,
                "warnings": 0,
                "skipped": 0,
                "total": 0,
                "result": [
                    "$[0].images: the items in the array must be unique"
                ]
            }
        );

        let result = UploadResult::new(0, 0, 0, 0, vec!["$[0].images: the items in the array must be unique".to_string()]);

        let r: UploadResult = serde_json::from_value(result_json).unwrap();
        assert_eq!(r, result);
    }
}
