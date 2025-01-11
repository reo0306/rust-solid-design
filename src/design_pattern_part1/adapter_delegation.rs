use std::collections::HashMap;

trait Target {
    fn get_csv_data(&self) -> String;
}

struct NewLibrary;

impl NewLibrary {
    fn new() -> Self {
        Self
    }

    fn get_json_data(&self) -> HashMap<String, String> {
        let json_data = HashMap::from([
        ("data1".to_string(), "json_dataA".to_string()),
        ("data2".to_string(), "json_dataB".to_string()),
        ("data3".to_string(), "json_dataC".to_string()),
        ("data4".to_string(), "json_dataD".to_string()),
        ]);

        json_data
    }
}

struct JsonToCsvAdapter {
    adaptee: NewLibrary,
}

impl JsonToCsvAdapter {
    fn new(adaptee: NewLibrary) -> Self {
        Self { adaptee }
    }
}

impl Target for JsonToCsvAdapter {
    fn get_csv_data(&self) -> String {
        let json_data = self.adaptee.get_json_data();
        let header: Vec<String> = json_data.clone().into_keys().collect();
        let header_ex = header.join(",") + "\n";
        let body: Vec<String> = json_data.clone().into_values().collect();
        let body_ex = body.join(",");
        let csv = format!("{}{}", header_ex, body_ex);

        csv
    }
}

pub struct AdapterDeleMain;

impl AdapterDeleMain {
    pub fn index() {
        let adaptee = NewLibrary::new();
        println!("=== Adapteeが提供するデータ ===");
        println!("{:?}", adaptee.get_json_data());

        let adapter = JsonToCsvAdapter::new(adaptee);
        println!("=== Adapterが提供するデータ ===");
        println!("{}", adapter.get_csv_data());
    }
}