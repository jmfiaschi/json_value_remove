extern crate serde_json;

use std::io;
use serde_json::Value;

/// Trait used to remove Json Value's element
pub trait Remove {
    /// Method use to remove element in Json Values
    fn remove(&mut self, json_pointer: &str) -> io::Result<Option<Value>>;
}

impl Remove for serde_json::Value {
    /// # Examples: Remove an element in a table
    /// ```
    /// use serde_json::Value;
    /// use json_value_remove::Remove;
    ///
    /// let mut array1: Value = serde_json::from_str(r#"{"my_table":["a","b","c"]}"#).unwrap();
    /// assert_eq!(Some(Value::String("a".to_string())), array1.remove("/my_table/0").unwrap());
    /// assert_eq!(r#"{"my_table":["b","c"]}"#, array1.to_string());
    /// ```
    /// # Examples: Remove a field from an object
    /// ```
    /// use serde_json::Value;
    /// use json_value_remove::Remove;
    ///
    /// let mut object1: Value = serde_json::from_str(r#"{"field1.0":{"field1.1":"value1.1","field1.2":"value1.2"},"field2.0":"value2.0"}"#).unwrap();
    /// assert_eq!(Some(Value::String("value1.2".to_string())), object1.remove("/field1.0/field1.2").unwrap());
    /// assert_eq!(r#"{"field1.0":{"field1.1":"value1.1"},"field2.0":"value2.0"}"#,object1.to_string());
    /// ```
    fn remove(&mut self, json_pointer: &str) -> io::Result<Option<Value>> {
        let fields: Vec<&str> = json_pointer.split("/").skip(1).collect();

        remove(self, fields)
    }
}

fn remove(json_value: &mut Value, fields: Vec<&str>) -> io::Result<Option<Value>> {
    if fields.is_empty() {
        return Ok(None);
    }

    let mut fields = fields.clone();
    let field = fields.remove(0);

    if field.is_empty() {
        return Ok(None);
    }

    match fields.is_empty() {
        true => match json_value {
            Value::Array(vec) => {
                let index = match field.parse::<usize>() {
                    Ok(index) => index,
                    Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("{}. Can't find the field '{}' in {}.", e, field, json_value.to_string())))
                };
                let len = vec.len();
                if index >= len {
                    return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("removal index (is {}) should be < len (is {}) from {}", index, len, json_value.to_string())))
                }
                Ok(Some(vec.remove(index)))
            },
            Value::Object(map) => Ok(map.remove(field)),
            _ => Ok(None)
        },
        false => match json_value.pointer_mut(format!("/{}", field).as_str()) {
            Some(json_targeted) => remove(json_targeted, fields),
            None => Ok(None)
        }
    }
}

#[cfg(test)]
mod serde_json_value_remove_test {
    use super::*;
    #[test]
    fn it_should_remove_element_in_array() {
        let mut array1: Value = serde_json::from_str(r#"{"my_table":["a","b","c"]}"#).unwrap();
        assert_eq!(Some(Value::String("a".to_string())), array1.remove("/my_table/0").unwrap());
        assert_eq!(r#"{"my_table":["b","c"]}"#, array1.to_string());
    }
    #[test]
    fn it_should_remove_element_in_object() {
        let mut object1: Value = serde_json::from_str(r#"{"field1.0":{"field1.1":"value1.1","field1.2":"value1.2"},"field2.0":"value2.0"}"#).unwrap();
        assert_eq!(Some(Value::String("value1.2".to_string())), object1.remove("/field1.0/field1.2").unwrap());
        assert_eq!(r#"{"field1.0":{"field1.1":"value1.1"},"field2.0":"value2.0"}"#,object1.to_string());
    }
    #[test]
    fn it_should_raise_exception_in_array_with_unavailable_index() {
        let mut array1: Value = serde_json::from_str(r#"{"my_table":["a","b"]}"#).unwrap();
        match array1.remove("/my_table/3") {
            Ok(_) => assert!(false, "Should raise an error because the element not exist"),
            Err(e) =>  assert_eq!("removal index (is 3) should be < len (is 2) from [\"a\",\"b\"]".to_string(), e.to_string())
        };
    }
    #[test]
    fn it_should_raise_exception_in_array_with_unavailable_field() {
        let mut array1: Value = serde_json::from_str(r#"{"my_table":["a","b"]}"#).unwrap();
        match array1.remove("/my_table/a") {
            Ok(_) => assert!(false, "Should raise an error because the element not exist"),
            Err(e) =>  assert_eq!("invalid digit found in string. Can't find the field 'a' in [\"a\",\"b\"].".to_string(), e.to_string())
        };
    }
    #[test]
    fn it_should_return_empty_if_field_not_exist() {
        let mut object1: Value = serde_json::from_str(r#"{"field1.0":{"field1.1":"value1.1","field1.2":"value1.2"}}"#).unwrap();
        assert_eq!(None, object1.remove("/field1.0/field1.3").unwrap());
    }
    #[test]
    fn it_should_return_empty_if_field_is_empty() {
        let mut object1: Value = serde_json::from_str(r#"{"field1.0":{"field1.1":"value1.1","field1.2":"value1.2"}}"#).unwrap();
        assert_eq!(None, object1.remove("/field1.0/").unwrap());
        assert_eq!(None, object1.remove("/field1.0/ ").unwrap());
        assert_eq!(None, object1.remove("/field1.0/ /field1.1").unwrap());
    }
}
