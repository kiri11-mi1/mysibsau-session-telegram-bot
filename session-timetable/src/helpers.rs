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

pub fn fetch_normal_time(time_value: &str) -> String {
    return time_value.replace("-", "");
}
