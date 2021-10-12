mod wrapper;
use wrapper::*;

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use crate::JsonStruct;

    // Struct used for testing
    #[derive(Serialize, Deserialize, Debug, Clone)]
    struct TestStruct {
        id: usize,
        name: String,
        bool: bool,
    }

    impl Default for TestStruct {
        fn default() -> Self {
            return TestStruct {
                id: 0,
                name: "".to_string(),
                bool: false,
            };
        }
    }

    // Extended Struct used for testing
    #[derive(Serialize, Deserialize, Debug, Clone)]
    struct NewTestStruct {
        id: usize,
        name: String,
        bool: bool,
        ref_id: usize,
    }

    impl Default for NewTestStruct {
        fn default() -> Self {
            return NewTestStruct {
                id: 0,
                name: "".to_string(),
                bool: false,
                ref_id: 6,
            };
        }
    }

    // Tests if creation is successful
    #[test]
    fn create_obj() {
        let st = TestStruct { id: 3, name: "HelloWorld".to_string(), bool: true };
        let _ = JsonStruct::new("./st.json", st);
        assert!(std::path::Path::new("./st.json").exists());
        std::fs::remove_file("./st.json").unwrap();
    }

    // Test if mutation is successfully written to disk after drop and if loading works
    #[test]
    fn save_and_load_after_drop() {
        let st = TestStruct { id: 3, name: "HelloWorld".to_string(), bool: true };
        let mut js = JsonStruct::new("./save_and_load.json", st);
        js.id = 5;
        drop(js);

        let jsn: JsonStruct<TestStruct> = JsonStruct::load("./save_and_load.json");
        let st = TestStruct { id: 3, name: "HelloWorld".to_string(), bool: true };
        assert_eq!(st.name, jsn.name);
        assert_eq!(jsn.id, 5);
        std::fs::remove_file("./save_and_load.json").unwrap();
    }

    // Tests if loading works after extending the struct
    #[test]
    fn load_updated_struct() {
        {
            let st = TestStruct { id: 3, name: "HelloWorld".to_string(), bool: true };
            let _ = JsonStruct::new("./updated_struct.json", st);
        }
        let js: JsonStruct<NewTestStruct> = JsonStruct::load("./updated_struct.json");
        assert_eq!(js.id, 3);
        assert_eq!(js.name, "HelloWorld");
        assert_eq!(js.bool, true);
        assert_eq!(js.ref_id, 6);
        std::fs::remove_file("./updated_struct.json").unwrap();
    }

    // Tests if loading works after reducing the struct
    #[test]
    fn load_reduced_struct() {
        {
            let mut st = NewTestStruct::default();
            st.id = 10;
            st.name = "changed".to_string();
            let _ = JsonStruct::new("./reduced.json", st);
        }
        let js: JsonStruct<TestStruct> = JsonStruct::load("./reduced.json");
        assert_eq!(js.id, 10);
        assert_eq!(js.name, "changed");
        std::fs::remove_file("./reduced.json").unwrap();
    }
}
