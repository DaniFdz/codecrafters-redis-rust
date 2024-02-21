use std::collections::HashMap;

#[derive(Clone)]
pub struct RedisValue {
    value: String,
}

#[derive(Default, Clone)]
pub struct RedisDatabase {
    data: HashMap<String, RedisValue>,
    size: i32,
    pub set_key: String,
    pub get_key: String,
}

impl RedisDatabase {
    pub fn new() -> RedisDatabase {
        RedisDatabase {
            data: HashMap::new(),
            size: 0,
            set_key: String::new(),
            get_key: String::new(),
        }
    }
    pub fn set_key(&mut self, key: String, value: String) -> String {
        self.data.insert(key, RedisValue { value });
        self.size += 1;
        "+OK\r\n".to_string()
    }

    pub fn get_key(&self, key: String) -> String {
        match self.data.get(&key) {
            Some(value) => format!("+{}\r\n", value.value),
            None => "$-1\r\n".to_string(),

        }
    }

    pub fn remove_key(&mut self, key: String) -> String {
        match self.data.remove(&key) {
            Some(_) => "-OK\r\n".to_string(),
            None => "$-1\r\n".to_string(),
        }
    }

}

