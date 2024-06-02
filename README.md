# haz
## description
haz is a simple config file format, that supports simple variable definitions.

it utilizes the `.haz` file extension

## usage
### files
so far `haz` only has a simple syntax that consists of:
```
type:variable_name=variable_definition;
```

it ignores white spaces so the following would just be the same as above:
```
type  :variable_name    =  variable_definition     ;
```

types can be: `bool`, `str`, and `num` (`num` is just a 32 bit integer)

### lib
essentially, you create a new instance using `Config::new(path)`, which read the file at that path. NOTE: the file must be created beforehand.
then you can access the variables that were parsed with `test_config.parser.values`. the values variable is just a hashmap where the key value is of type `String` and the key is just the custom enum `Value` as defined in parser.

enum definition:
```rust
pub enum Value {
    String(String),
    Number(i32),
    Bool(bool),
}
```

example usage:
```rust
use haz::config::Config;
use haz::parser::Value;

fn haz_usage() {
    let mut config: Config = Config::new("path/to/file.haz").expect("Failed to open .haz config file");
    let new_var = config.parser.values.get("test").unwrap();
    println!("{:?}", new_var);
}
```
