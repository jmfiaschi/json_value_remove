# json_value_remove

[![Linter](https://github.com/jmfiaschi/json_value_remove/workflows/Lint/badge.svg)](https://github.com/jmfiaschi/json_value_remove/actions/workflows/lint.yml)
[![Actions Status](https://github.com/jmfiaschi/json_value_remove/workflows/CI/badge.svg)](https://github.com/jmfiaschi/json_value_remove/actions/workflows/ci.yml)
[![semantic-release](https://img.shields.io/badge/%20%20%F0%9F%93%A6%F0%9F%9A%80-semantic--release-e10079.svg)](https://github.com/semantic-release/semantic-release)

Give an interface to remove element into a json_serde::Value.

## Installation

 ```Toml
[dependencies]
json_value_remove = "1.0"
```

## Usage

Remove in an array:

```rust
extern crate json_value_remove;

use json_value_remove::Remove;
use serde_json::Value;

{
    let mut array1: Value = serde_json::from_str(r#"{"my_table":["a","b","c"]}"#).unwrap();
    assert_eq!(Some(Value::String("a".to_string())), array1.remove("/my_table/0").unwrap());
    assert_eq!(r#"{"my_table":["b","c"]}"#, array1.to_string());
}
```

Remove in an objects:

```rust
extern crate json_value_remove;

use json_value_remove::Remove;
use serde_json::Value;

{
    let mut object1: Value = serde_json::from_str(r#"{"field1.0":{"field1.1":"value1.1","field1.2":"value1.2"},"field2.0":"value2.0"}"#).unwrap();
    assert_eq!(Some(Value::String("value1.2".to_string())), object1.remove("/field1.0/field1.2").unwrap());
    assert_eq!(r#"{"field1.0":{"field1.1":"value1.1"},"field2.0":"value2.0"}"#,object1.to_string());
}
```

## Useful link

* [Benchmark report](https://jmfiaschi.github.io/json_value_remove/bench/main/)
* [Package](https://crates.io/crates/json_value_remove)

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

[Apache](https://choosealicense.com/licenses/apache-2.0/)
[MIT](https://choosealicense.com/licenses/mit/)
