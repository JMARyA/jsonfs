use std::fs::read;
use std::ops::{Deref, DerefMut};
use serde::Serialize;
use serde::de::DeserializeOwned;

/// Wrapper around a Struct which gets saved to disk
pub struct JsonStruct<T: Serialize + DeserializeOwned + Default> {
    _json_path: String,
    _struct: T,
}

impl<T: Serialize + DeserializeOwned + Default> Drop for JsonStruct<T> {
    fn drop(&mut self) {
        self.save();
    }
}

impl<T: Serialize + DeserializeOwned + Default> Deref for JsonStruct<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        return &self._struct;
    }
}

impl<T: Serialize + DeserializeOwned + Default> DerefMut for JsonStruct<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self._struct;
    }
}

impl<T: Serialize + DeserializeOwned + Default> JsonStruct<T> {
    /// This method creates a new JSON File for the given struct at given location
    pub fn new(path: &str, obj: T) -> JsonStruct<T> {
        return JsonStruct {
            _json_path: path.to_string(),
            _struct: obj,
        };
    }

    /// This loads a struct at the given location. If the JSON Data isn't an exact match with the defined struct, default values are used for everything missing.
    pub fn load(path: &str) -> JsonStruct<T> {
        let obj: Result<T, _> = serde_json::from_slice(&*read(path).unwrap());
        return match obj {
            Ok(obj) => {
                JsonStruct {
                    _json_path: path.to_string(),
                    _struct: obj,
                }
            }
            Err(_) => {
                let def = T::default();
                let mut def_json: serde_json::Value = serde_json::from_slice(&*serde_json::to_vec(&def).unwrap()).unwrap();
                let new_json: serde_json::Value = serde_json::from_slice(&*read(path).unwrap()).unwrap();
                Self::merge(&mut def_json, new_json);
                let obj: T = serde_json::from_slice(&serde_json::to_vec(&def_json).unwrap()).unwrap();
                JsonStruct {
                    _json_path: path.to_string(),
                    _struct: obj,
                }
            }
        };
    }

    /// Function for merging actual with default values
    fn merge(a: &mut serde_json::Value, b: serde_json::Value) {
        match (a, b) {
            (a @ &mut serde_json::Value::Object(_), serde_json::Value::Object(b)) => {
                let a = a.as_object_mut().unwrap();
                for (k, v) in b {
                    if v.is_array() && a.contains_key(&k) && a.get(&k).as_ref().unwrap().is_array() {
                        let mut _a = a.get(&k).unwrap().as_array().unwrap().to_owned();
                        _a.append(&mut v.as_array().unwrap().to_owned());
                        a[&k] = serde_json::Value::from(_a);
                    } else {
                        Self::merge(a.entry(k).or_insert(serde_json::Value::Null), v);
                    }
                }
            }
            (a, b) => *a = b,
        }
    }

    /// Explicitly serializes the struct and saves it
    pub fn save(&self) {
        let _ = std::fs::write(&self._json_path,
                               serde_json::to_vec(&self._struct).unwrap(),
        );
    }
}
