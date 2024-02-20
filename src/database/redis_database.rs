use std::collections::HashMap;

pub struct RedisValue {
    value: String,
}

#[derive(Default)]
pub struct RedisDatabase {
    data: HashMap<String, RedisValue>,
    pub set_key: String,
    pub get_key: String,
}

impl RedisDatabase {
    pub fn new() -> RedisDatabase {
        RedisDatabase {
            data: HashMap::new(),
            set_key: String::new(),
            get_key: String::new(),
        }
    }
    pub fn set_key(&mut self, key: String, value: String) -> String {
        self.data.insert(key, RedisValue { value });
        "+OK\r\n".to_string()
    }

    pub fn get_key(&self, key: String) -> String {
        match self.data.get(&key) {
            Some(value) => format!("+{}\r\n", value.value),
            None => "$-1\r\n".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_redis_database_set() {
        let mut db = RedisDatabase::new();
        assert_eq!(db.set_key("foo".to_string(), "bar".to_string()), "+OK\r\n");
    }

    #[test]
    fn test_redis_database_get() {
        let mut db = RedisDatabase::new();
        db.set_key("foo".to_string(), "bar".to_string());
        assert_eq!(db.get_key("foo".to_string()), "+bar\r\n");
        assert_eq!(db.get_key("baz".to_string()), "$-1\r\n");
    }
}
