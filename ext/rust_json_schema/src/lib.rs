use magnus::{
    exception::ExceptionClass, function, gc::register_mark_object, method, prelude::*, value::Lazy,
    wrap, RModule, Ruby,
};

extern crate serde_json;
use jsonschema::JSONSchema;

#[wrap(class = "RustJSONSchema::Validator")]
struct Validator {
    schema: JSONSchema,
}

impl Validator {
    fn new(ruby: &Ruby, json: String) -> Result<Validator, magnus::Error> {
        let value: serde_json::Value = match serde_json::from_str(&json) {
            Ok(value) => value,
            Err(error) => {
                return Err(magnus::Error::new(
                    ruby.get_inner(&JSON_PARSE_ERROR),
                    error.to_string(),
                ))
            }
        };

        let schema = match JSONSchema::options().compile(&value) {
            Ok(schema) => schema,
            Err(error) => {
                return Err(magnus::Error::new(
                    ruby.get_inner(&SCHEMA_PARSE_ERROR),
                    error.to_string(),
                ))
            }
        };

        Ok(Validator { schema })
    }

    fn is_valid(&self, json: String) -> bool {
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();

        self.schema.is_valid(&value)
    }

    fn validate(&self, json: String) -> Vec<String> {
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
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

static JSON_PARSE_ERROR: Lazy<ExceptionClass> = Lazy::new(|ruby| {
    let ex = ruby
        .class_object()
        .const_get::<_, RModule>("RustJSONSchema")
        .unwrap()
        .const_get("JSONParseError")
        .unwrap();
    // ensure `ex` is never garbage collected (e.g. if constant is
    // redefined) and also not moved under compacting GC.
    register_mark_object(ex);
    ex
});

static SCHEMA_PARSE_ERROR: Lazy<ExceptionClass> = Lazy::new(|ruby| {
    let ex = ruby
        .class_object()
        .const_get::<_, RModule>("RustJSONSchema")
        .unwrap()
        .const_get("SchemaParseError")
        .unwrap();
    // ensure `ex` is never garbage collected (e.g. if constant is
    // redefined) and also not moved under compacting GC.
    register_mark_object(ex);
    ex
});

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), magnus::Error> {
    let module = ruby.define_module("RustJSONSchema")?;
    let class = module.define_class("Validator", ruby.class_object())?;

    class.define_singleton_method("new", function!(Validator::new, 1))?;

    class.define_method("valid?", method!(Validator::is_valid, 1))?;
    class.define_method("validate", method!(Validator::validate, 1))?;

    // Ensure defined at load time
    Lazy::force(&JSON_PARSE_ERROR, ruby);
    Lazy::force(&SCHEMA_PARSE_ERROR, ruby);

    Ok(())
}
