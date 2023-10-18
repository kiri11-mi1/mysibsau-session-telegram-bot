use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::helpers::{fetch_day_name, fetch_lesson_name};
use std::convert::From;

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

#[derive(Serialize, Deserialize, Debug)]
pub struct Exam {
    pub proffessor: String,
    pub lesson: String,
    pub room: String,
    pub day_week: String,
    pub time: String,
    pub date: String,
}

impl From<Map<String, Value>> for Exam {
    // Сериализатор для экзамена
    fn from(value: Map<String, Value>) -> Self {
        Self {
            proffessor: value["employee_name_init"].to_string().replace("\"", ""),
            lesson: fetch_lesson_name(value["lesson"].clone()),
            room: value["place"]
                .to_string()
                .replace("\\\"", "")
                .replace("\"", ""),
            date: value["date"].to_string().replace("\"", ""),
            day_week: fetch_day_name(value["day_week"].as_str().unwrap()),
            time: value["time"].to_string().replace("-", "").replace("\"", ""),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct List<T> {
    pub data: Vec<T>,
}

impl<T: From<Map<String, Value>>> From<Vec<Map<String, Value>>> for List<T> {
    // Сериализатор для списка экзаменов
    fn from(items: Vec<Map<String, Value>>) -> Self {
        let mut result = vec![];
        for v in items {
            result.push(T::from(v))
        }
        return List { data: result };
    }
}
