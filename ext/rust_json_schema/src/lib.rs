use magnus::{function, method, prelude::*, wrap, Error, Ruby};

use jsonschema::JSONSchema;
use serde_json::Value;

#[wrap(class = "RustJSONSchema::Validator")]
struct Validator {
    schema: JSONSchema,
}

impl Validator {
    fn new(json: String) -> Validator {
        let value: Value = match serde_json::from_str(&json) {
            Ok(value) => value,
            Err(error) => {
                panic!("Could not parse JSON: {}", error);
            }
        };

        Validator {
            schema: JSONSchema::options().compile(&value).unwrap(),
        }
    }

    fn is_valid(&self, json: String) -> bool {
        let value: Value = serde_json::from_str(&json).unwrap();

        self.schema.is_valid(&value)
    }

    fn validate(&self, json: String) -> Vec<String> {
        let value: Value = serde_json::from_str(&json).unwrap();
        let mut errors: Vec<String> = vec![];

        if let Err(validation_errors) = self.schema.validate(&value) {
            for error in validation_errors {
                let path = match format!("{}", error.instance_path).as_str() {
                    "" => "/".to_string(),
                    p => p.to_string(),
                };

                errors.push(format!("path \"{}\": {}", path, error));
            }
        }

        errors
    }
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.define_module("RustJSONSchema")?;
    let class = module.define_class("Validator", ruby.class_object())?;

    class.define_singleton_method("new", function!(Validator::new, 1))?;

    class.define_method("is_valid", method!(Validator::is_valid, 1))?;
    class.define_method("validate", method!(Validator::validate, 1))?;

    Ok(())
}
