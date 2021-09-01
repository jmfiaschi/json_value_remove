extern crate serde_json;

use serde_json::Value;

/// Trait used to remove Json Value's element
pub trait Remove {
    /// Method use to remove element in Json Values
    fn remove(&mut self, json_pointer: &str);
}

impl Remove for serde_json::Value {
    /// # Examples: Remove an element in a table
    /// ```
    /// use serde_json::Value;
    /// use json_value_remove::Remove;
    ///
    /// let mut array1: Value = serde_json::from_str(r#"{"my_table":["a","b","c"]}"#).unwrap();
    /// array1.remove("/my_table/0");
    /// assert_eq!(r#"{"my_table":["b","c"]}"#, array1.to_string());
    /// ```
    /// # Examples: Remove a field from an object
    /// ```
    /// use serde_json::Value;
    /// use json_value_remove::Remove;
    ///
    /// let mut object1: Value = serde_json::from_str(r#"{"field1.0":{"field1.1":"value1.1","field1.2":"value1.2"},"field2.0":"value2.0"}"#).unwrap();
    /// object1.remove("/field1.0/field1.2");
    /// assert_eq!(r#"{"field1.0":{"field1.1":"value1.1"},"field2.0":"value2.0"}"#,object1.to_string());
    /// ```
    fn remove(&mut self, json_pointer: &str) {
        let fields: Vec<&str> = json_pointer.split("/").skip(1).collect();

        remove(self, fields);
    }
}

fn remove(json_value: &mut Value, fields: Vec<&str>) -> () {
    if fields.is_empty() {
        return ();
    }

    let mut fields = fields.clone();
    let field = fields.remove(0);

    if field.is_empty() {
        return ();
    }

    match fields.is_empty() {
        true => match json_value {
            Value::Array(vec) => {
                vec.remove(field.parse::<usize>().unwrap());
            },
            Value::Object(map) => {
                map.remove(field);
            },
            _ => ()
        },
        false => match json_value.pointer_mut(format!("/{}", field).as_str()) {
            Some(json_targeted) => remove(json_targeted, fields),
            None => ()
        }
    };
}

#[cfg(test)]
mod serde_json_value_remove_test {
    use super::*;
    #[test]
    fn it_should_remove_element_in_array() {
        let mut array1: Value = serde_json::from_str(r#"{"my_table":["a","b","c"]}"#).unwrap();
        array1.remove("/my_table/0");
        assert_eq!(r#"{"my_table":["b","c"]}"#, array1.to_string());
    }
    #[test]
    fn it_should_remove_element_in_object() {
        let mut object1: Value = serde_json::from_str(r#"{"field1.0":{"field1.1":"value1.1","field1.2":"value1.2"},"field2.0":"value2.0"}"#).unwrap();
        object1.remove("/field1.0/field1.2");
        assert_eq!(r#"{"field1.0":{"field1.1":"value1.1"},"field2.0":"value2.0"}"#,object1.to_string());
    }
}
