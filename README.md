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
    let mut first_json_value: Value = serde_json::from_str(r#"["a","b"]"#).unwrap();
    let secound_json_value: Value = serde_json::from_str(r#"["b","c"]"#).unwrap();
    first_json_value.merge(secound_json_value);
    assert_eq!(r#"["a","b","c"]"#, first_json_value.to_string());
}
```

Remove in an objects:

```rust
extern crate json_value_remove;

use json_value_remove::Remove;
use serde_json::Value;

{
    let mut first_json_value: Value =
        serde_json::from_str(r#"[{"value":"a"},{"value":"b"}]"#).unwrap();
    let secound_json_value: Value =
        serde_json::from_str(r#"[{"value":"b"},{"value":"c"}]"#).unwrap();
    first_json_value.merge(secound_json_value);
    assert_eq!(
        r#"[{"value":"a"},{"value":"b"},{"value":"c"}]"#,
        first_json_value.to_string()
    );
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
