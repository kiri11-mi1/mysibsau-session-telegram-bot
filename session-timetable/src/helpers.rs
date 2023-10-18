use serde_json::Value;

pub fn fetch_day_name(number_day: &str) -> String {
    match number_day {
        "1" => "Понедельник".to_string(),
        "2" => "Вторник".to_string(),
        "3" => "Среда".to_string(),
        "4" => "Четверг".to_string(),
        "5" => "Пятница".to_string(),
        "6" => "Суббота".to_string(),
        _ => "Не известно".to_string(),
    }
}

pub fn fetch_lesson_name(lesson: Value) -> String {
    let lesson_arr = lesson.as_array();
    if lesson_arr.is_none() {
        return "".to_string();
    }
    let lesson_arr = lesson_arr.unwrap();
    if lesson_arr.len() != 2 {
        return "".to_string();
    }
    return lesson_arr[1].to_string().replace("\"", "");
}
