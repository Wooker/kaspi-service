use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct KaspiCategoryAttribute {
    pub code: String,
    pub r#type: String,
    pub multiValued: bool,
    pub mandatory: bool,
}

#[derive(Serialize, Deserialize, Eq, Hash, PartialEq, Debug, Clone)]
#[serde(untagged)]
pub enum AttributeValue {
    String(String),
    Boolean(bool)
}

#[derive(Serialize, Deserialize, Eq, Hash, PartialEq, Debug, Clone)]
pub struct Attribute {
    pub code: String,
    pub value: AttributeValue,
}

#[cfg(test)]
mod tests {
    use crate::entities::attribute::{Attribute, AttributeValue};

    #[test]
    fn attribute_to_json() {
        let attribute = Attribute {
            code: String::from("Wigs and hairpieces*Harakteristiki.wigs and hairpieces*purpose"),
            value: AttributeValue::String(String::from("zhenskiy"))
        };

        let json = serde_json::to_value(attribute).unwrap();

        assert_eq!(
            json, serde_json::json!({
                "code": "Wigs and hairpieces*Harakteristiki.wigs and hairpieces*purpose",
                "value": "zhenskiy"
            })
        );
    }

    #[test]
    fn json_to_attribute() {
        let value = serde_json::json!({
            "code": "Wigs and hairpieces*Harakteristiki.wigs and hairpieces*purpose",
            "value": "zhenskiy"
        });

        let attribute_from_json: Attribute = serde_json::from_value(value).unwrap();
        let attribute: Attribute = Attribute {
            code: "Wigs and hairpieces*Harakteristiki.wigs and hairpieces*purpose".to_string(),
            value: AttributeValue::String("zhenskiy".to_string())
        };

        assert_eq!(attribute, attribute_from_json);
    }
}
