use serde::{Deserialize, Serialize};
use crate::entities::{
    attribute::Attribute,
    upload_result::UploadResult,
};
use uuid::Uuid;
use std::fmt;

/*
#[derive(Error, Debug)]
pub enum ProductError {
    #[error("Could not parse input")]
    Parsing(#[from] FromStr::Error),
    CategoryFetch,
    Images,
}
*/

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
struct ProductImage {
    url: String
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Product {
    sku: String,
    title: String,
    brand: String,
    category: String,
    description: String,
    attributes: Vec<Attribute>,
    images: Vec<ProductImage>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Record {
    id: Uuid,
    product: Product,
    code: String,
    status: String,
    result: Option<UploadResult>,
}

impl Record {
    pub fn sku(&self) -> String {
        self.product.sku.clone()
    }

    pub fn show_all() {}
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
        attribute::{Attribute, AttributeValue},
    };

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
    fn record_to_json() {
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
    fn json_to_record() {
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
}
