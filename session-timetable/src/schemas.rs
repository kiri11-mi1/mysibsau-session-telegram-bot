use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Serialize, Deserialize, Debug)]
pub struct Group {
    pub id: i64,
    pub name: String,
}

impl From<Map<String, Value>> for Group {
    // Сериализатор для группы
    fn from(value: Map<String, Value>) -> Self {
        Self {
            id: value["id"].as_i64().unwrap(),
            name: value["name"].to_string(),
        }
    }
}

pub struct GroupList {
    pub data: Vec<Group>,
}

impl From<Vec<Map<String, Value>>> for GroupList {
    // Сериализатор для списка групп
    fn from(items: Vec<Map<String, Value>>) -> Self {
        let mut result = vec![];
        for v in items {
            result.push(Group::from(v))
        }
        return GroupList { data: result };
    }
}
