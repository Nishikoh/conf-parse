use configparser::ini::Ini;
use std::collections::HashMap;
use std::env;
use std::error::Error;

// 各タイプのハッシュマップのstruct 定義
#[derive(Debug)]
struct TypedConfig {
    integer_config: HashMap<String, i128>,
    float_config: HashMap<String, f64>,
    string_config: HashMap<String, String>,
    bool_config: HashMap<String, bool>,
    none_config: HashMap<String, ()>,
}

impl TypedConfig {
    // 各タイプのハッシュマップを初期化する
    fn new() -> Self {
        TypedConfig {
            integer_config: HashMap::new(),
            float_config: HashMap::new(),
            string_config: HashMap::new(),
            bool_config: HashMap::new(),
            none_config: HashMap::new(),
        }
    }

    fn raw_to_typed(raw_config: HashMap<String, HashMap<String, Option<String>>>) -> TypedConfig {
        let mut typed_config: TypedConfig = TypedConfig::new();
        for (_section, prop) in raw_config {
            for (key, value) in prop.iter() {
                let key = key.to_string();
                typed_config.insert_typed_value(key, value);
            }
        }
        typed_config
    }

    fn insert_integer_value(&mut self, key: String, value: i128) {
        self.integer_config.insert(key, value);
    }
    fn insert_float_value(&mut self, key: String, value: f64) {
        self.float_config.insert(key, value);
    }
    fn insert_string_value(&mut self, key: String, value: String) {
        self.string_config.insert(key, value);
    }
    fn insert_bool_value(&mut self, key: String, value: bool) {
        self.bool_config.insert(key, value);
    }
    fn insert_none_value(&mut self, key: String) {
        self.none_config.insert(key, ());
    }

    fn insert_typed_value(&mut self, key: String, value: &Option<String>) {
        if value.is_none() {
            self.insert_none_value(key);
        } else {
            let typed_value = TypedValue::new(value.clone().unwrap());
            match typed_value {
                TypedValue {
                    integer_value: Some(integer_value),
                    float_value: _,
                    string_value: _,
                    bool_value: _,
                } => {
                    self.insert_integer_value(key, integer_value);
                }
                TypedValue {
                    integer_value: _,
                    float_value: Some(float_value),
                    string_value: _,
                    bool_value: _,
                } => {
                    self.insert_float_value(key, float_value);
                }
                TypedValue {
                    integer_value: _,
                    float_value: _,
                    string_value: _,
                    bool_value: Some(bool_value),
                } => {
                    self.insert_bool_value(key, bool_value);
                }
                TypedValue {
                    integer_value: None,
                    float_value: None,
                    string_value: Some(string_value),
                    bool_value: None,
                } => {
                    self.insert_string_value(key, string_value);
                }
                _ => panic!(),
            }
        }
    }
}

// 各タイプの構造体を定義
#[derive(Debug)]
struct TypedValue {
    integer_value: Option<i128>,
    float_value: Option<f64>,
    string_value: Option<String>,
    bool_value: Option<bool>,
}

impl TypedValue {
    fn new(value: String) -> Self {
        TypedValue {
            integer_value: value.parse::<i128>().ok(),
            float_value: value.parse::<f64>().ok(),
            string_value: Some(value.to_string()),
            bool_value: value.parse::<bool>().ok(),
        }
    }
}

fn main() -> Result<(), Box<(dyn Error + 'static)>> {
    let args: Vec<String> = env::args().collect();
    let mut config = Ini::new();

    let raw_config = config.load(&args[1])?;
    let typed_config = TypedConfig::raw_to_typed(raw_config);
    println!("{:#?}", typed_config);

    Ok(())
}
