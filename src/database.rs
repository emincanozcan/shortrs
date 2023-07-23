use std::{collections::HashMap, sync::Mutex, io, path::Path, fs::File,io::Write};

pub struct Database {
  url_kv: std::sync::Mutex<HashMap<String, String>>,
  file_path: String,
}

impl Database {
  pub fn new(file_path: &str) -> io::Result<Self> {
      let db = Database {
          url_kv: Mutex::new(HashMap::new()),
          file_path: file_path.to_string(),
      };
      if Path::new(file_path).exists() {
          let file = File::open(file_path)?;
          let data: HashMap<String, String> = serde_json::from_reader(file)?;
          *db.url_kv.lock().unwrap() = data;
       }
      Ok(db)
  }

  pub fn save_to_file(&self) -> io::Result<()> {
      let file = File::create(&self.file_path)?;
      let data = self.url_kv.lock().unwrap();
      let json = serde_json::to_string(&data.clone())?;
      let mut writer = io::BufWriter::new(file);
      writer.write_all(json.as_bytes())?;
      writer.flush()?;
      Ok(())
  }

  pub fn store_kv(&self, key: String, value: String) -> io::Result<()> {
      self.url_kv.lock().unwrap().insert(key, value);
      self.save_to_file()
  }

  pub fn get_value(&self, key: &str) -> Option<String> {
      self.url_kv.lock().unwrap().get(key).cloned()
  }

  pub fn get_all(&self) -> Vec<(String, String)> {
      self.url_kv
          .lock()
          .unwrap()
          .iter()
          .map(|(key, value)| (key.to_owned(), value.to_owned()))
          .collect::<Vec<(String, String)>>()
  }
}
