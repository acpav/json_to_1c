use std::str::FromStr;

use chrono::{NaiveDateTime, DateTime};

use serde_json::Value;

pub struct ParseJson1c {
    
    pub random : bool,
    pub show_area : bool,
    pub count_to_wrap_array : usize,
}

impl ParseJson1c {

    pub fn new(random:bool, show_area:bool, count_to_wrap_array:usize) -> Self
    {
        ParseJson1c{random, show_area, count_to_wrap_array}
    }

    pub fn parse(&self, text: &str) -> Vec<String>{

        let mut result_str: Vec<String> = Vec::new();

        let json_data = serde_json::from_str(text).expect("Не парсится json");
    
        let key = "Данные";
        self.read_json(&json_data, key, "", &mut result_str);
    
        result_str

    }

    fn read_json(&self, json_data:&Value, key:&str, name_array:&str, result_str: &mut Vec<String>)
    {
        match json_data {
    
            Value::Object(map) => {
                result_str.push(self.area_begin(key));
                result_str.push(format!("\t{key} = Новый Структура;"));
    
                for val in map
                {
                    if !Value::is_object(val.1) && !Value::is_array(val.1)
                    { 
                        result_str.push(format!("\t{0}.Вставить(\"{1}\", {2});{3}", key, val.0, self.to_1c_val(val.1), self.to_random(val.1)));
                    }
                }
    
                for val in map
                {
                    if Value::is_object(val.1)
                    { 
                        self.read_json(val.1, format!("Ст_{}", val.0).as_str(), "", result_str);
                        result_str.push(format!("\t{0}.Вставить(\"{1}\", Ст_{2});", key, val.0, val.0));
                    }
                }
    
                for val in map
                {
                    if Value::is_array(val.1)
                    { 
                        self.read_json(val.1, format!("М_{}", val.0).as_str(), "", result_str);
                        result_str.push(format!("\t{0}.Вставить(\"{1}\", М_{2});", key, val.0, val.0));
                    }
                }
    
                if !name_array.is_empty() {
                    result_str.push(format!("\t{0}.Добавить({1});", name_array, key));
                }
                result_str.push(self.area_end());
    
            }
    
            Value::Array(arr) => {
    
                result_str.push(self.area_begin(key));
                result_str.push(format!("\t{} = Новый Массив;", key));
    
                if arr.len() > self.count_to_wrap_array
                {
                    result_str.push(format!("\tДля а = 1 По {0} Цикл", arr.len()));
                    self.read_json(arr.first().unwrap(), format!("Эл_{key}").as_str(), key, result_str);
                    result_str.push("\tКонецЦикла;".to_string());
                }
                else {
                    for val in arr.into_iter()
                    {
                        self.read_json(val, format!("Эл_{key}").as_str(), key, result_str);
                    }
                }
                
                result_str.push(self.area_end());

            }
    
            _ => {
                if !name_array.is_empty() {
                    result_str.push(format!("\t{0}.Добавить({1});{2}", name_array, self.to_1c_val(json_data), self.to_random(json_data)));
                }
                else
                {
                    result_str.push(format!("\t{0} = {1};{2}", key, self.to_1c_val(json_data), self.to_random(json_data)));
                }
            }
    
        }
    
    }
    
    fn to_random(&self, val:&Value) -> String
    {
    
        if !self.random{
            return String::new();
        }

        match val {
            
            Value::Number(n) =>
            {
                let dev = if n.is_f64(){
                    ", 2)"
                }
                else {
                    ")"
                };
    
                return if n.as_f64().unwrap() > 0.0
                {
                    format!(" // ЮТест.Данные().СлучайноеПоложительноеЧисло({0}{dev}",
                        (n.as_f64().unwrap() as i32).checked_ilog10().unwrap_or(0) + 1)
                }
                else if n.as_f64().unwrap() < 0.0 {
                    format!(" // ЮТест.Данные().СлучайноеОтрицательноеЧисло({0}{dev}",
                        (-n.as_f64().unwrap() as i32).checked_ilog10().unwrap_or(0) + 1)
                }
                else {
                    String::new()
                };
            }
    
            Value::String(s) =>
            {
                if is_uid(s) {
                    return format!(" // ЮТест.Данные().СлучайныйИдентификатор()");
                }
                else if NaiveDateTime::from_str(s).is_ok()
                    || DateTime::parse_from_rfc2822(s).is_ok()
                    || DateTime::parse_from_rfc3339(s).is_ok()
                {
                    return format!(" // ЮТест.Данные().СлучайнаяДатаВПрошлом(-10, \"дней\")");
                }
    
                else if s.chars().count() > 0 {
                    return format!(" // ЮТест.Данные().СлучайнаяСтрока({0})", s.chars().count());
                }
                else {
                    return String::new();
                }
    
            }
    
            _ => {return String::new();}
        }
    
    }
    
    fn to_1c_val(&self, val:&Value) -> String
    {
        match val {
            Value::String(s)=>
            {
                return format!("\"{}\"", s.replace("\"", "\"\"").replace('\n', "\n|"));
            }
    
            Value::Null =>
            {
                return String::from_str("Неопределено").unwrap();
            }

            _ => {return val.to_string();}
        }
    }

    fn area_begin(&self, key:&str) -> String
    {
        return if self.show_area
        {
            format!("#Область {key}")
        }
        else
        {
            String::new()
        };
    }

    fn area_end(&self) -> String
    {
        return if self.show_area
        {
            "#КонецОбласти".to_string()
        }
        else
        {
            String::new()
        };
    }

}

pub fn is_uid(value: &String) -> bool
{
    let value_trim = value.trim();
    if value_trim.len() == 36
        && value_trim.get(8..9) == Some("-")
        && value_trim.get(13..14) == Some("-")
        && value_trim.get(18..19) == Some("-")
        && value_trim.get(23..24) == Some("-")
        && value_trim.replace("-", "").len() == 32
        && value_trim.chars().all(|c| c.is_ascii_hexdigit() || c == '-')
    {
        return true;
    }
    else
    {
        return false;
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn it_works() {
        let value = String::from_str("test").unwrap();
        assert!(!is_uid(&value), "Not uid");

        let value = String::from_str("e6f170c2-934a-11ee-b812-b2eac8ff4de4").unwrap();
        assert!(is_uid(&value), "Is't UID");

        let value = String::from_str("testtesttesttesttesttesttesttesttest").unwrap();
        assert!(!is_uid(&value), "Not UID, len 36");

        let value = String::from_str("------------------------------------").unwrap();
        assert!(!is_uid(&value), "Not UID, len 36, -----");

        let parcer = ParseJson1c::new(true, false, 1);

        assert_eq!(parcer.to_random(&Value::from(10)), " // ЮТест.Данные().СлучайноеПоложительноеЧисло(2)", "Число 10");
        assert_eq!(parcer.to_random(&Value::from(-10)), " // ЮТест.Данные().СлучайноеОтрицательноеЧисло(2)", "Число -10");
        assert_eq!(parcer.to_random(&Value::from(107.47)), " // ЮТест.Данные().СлучайноеПоложительноеЧисло(3, 2)", "Число 107.47");
        assert_eq!(parcer.to_random(&Value::from(-107.47)), " // ЮТест.Данные().СлучайноеОтрицательноеЧисло(3, 2)", "Число -107.47");

    }
}
