extern crate serde_json;

use jsonschema::Draft;
use magnus::{
    exception::ExceptionClass,
    function,
    gc::register_mark_object,
    method,
    prelude::*,
    scan_args::{get_kwargs, scan_args},
    value::Lazy,
    wrap, Error, RHash, RModule, Ruby, StaticSymbol, Value,
};

#[wrap(class = "RustJSONSchema::Validator")]
struct Validator {
    schema: jsonschema::Validator,
    draft: Draft,
}

impl Validator {
    fn new(args: &[Value]) -> Result<Validator, Error> {
        let args = scan_args::<_, (), (), (), _, ()>(args)?;
        let (json,): (String,) = args.required;
        let kwargs = get_kwargs::<_, (), (Option<Value>,), ()>(args.keywords, &[], &["draft"])?;
        let (draft_arg,): (Option<Value>,) = kwargs.optional;

        let draft = match draft_arg {
            Some(draft) => match draft.to_string().to_lowercase().as_str() {
                "draft4" => Draft::Draft4,
                "draft6" => Draft::Draft6,
                "draft7" => Draft::Draft7,
                "draft201909" => Draft::Draft201909,
                "draft202012" => Draft::Draft202012,
                _ => {
                    return Err(Error::new(
                        Self::ruby().get_inner(&INVALID_OPTIONS_ERROR),
                        format!("invalid draft: '{}'", draft.to_string()),
                    ))
                }
            },
            None => jsonschema::Draft::default(),
        };

        let value: serde_json::Value = match serde_json::from_str(&json) {
            Ok(value) => value,
            Err(error) => {
                return Err(Error::new(
                    Self::ruby().get_inner(&JSON_PARSE_ERROR),
                    error.to_string(),
                ))
            }
        };

        let mut schema = jsonschema::Validator::options();
        schema.with_draft(draft);

        let schema = match schema.build(&value) {
            Ok(schema) => schema,
            Err(error) => {
                return Err(Error::new(
                    Self::ruby().get_inner(&SCHEMA_PARSE_ERROR),
                    error.to_string(),
                ))
            }
        };

        Ok(Validator { schema, draft })
    }

    fn is_valid(&self, json: String) -> Result<bool, Error> {
        let value: serde_json::Value = match serde_json::from_str(&json) {
            Ok(value) => value,
            Err(error) => {
                return Err(Error::new(
                    Self::ruby().get_inner(&JSON_PARSE_ERROR),
                    error.to_string(),
                ));
            }
        };

        Ok(self.schema.is_valid(&value))
    }

    fn options(&self) -> Result<RHash, Error> {
        let options = RHash::new();

        options
            .aset(
                StaticSymbol::new("draft"),
                StaticSymbol::new(format!("{:?}", self.draft).to_lowercase()),
            )
            .unwrap();

        Ok(options)
    }

    fn validate(&self, json: String) -> Result<Vec<String>, Error> {
        let value: serde_json::Value = match serde_json::from_str(&json) {
            Ok(value) => value,
            Err(error) => {
                return Err(Error::new(
                    Self::ruby().get_inner(&JSON_PARSE_ERROR),
                    error.to_string(),
                ))
            }
        };

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

        Ok(errors)
    }

    fn ruby() -> Ruby {
        Ruby::get().unwrap()
    }
}

static JSON_PARSE_ERROR: Lazy<ExceptionClass> = Lazy::new(|ruby| {
    let ex = ruby
        .class_object()
        .const_get::<_, RModule>("RustJSONSchema")
        .unwrap()
        .const_get("JSONParseError")
        .unwrap();
    register_mark_object(ex); // avoid GC
    ex
});

static SCHEMA_PARSE_ERROR: Lazy<ExceptionClass> = Lazy::new(|ruby| {
    let ex = ruby
        .class_object()
        .const_get::<_, RModule>("RustJSONSchema")
        .unwrap()
        .const_get("SchemaParseError")
        .unwrap();
    register_mark_object(ex); // avoid GC
    ex
});

static INVALID_OPTIONS_ERROR: Lazy<ExceptionClass> = Lazy::new(|ruby| {
    let ex = ruby
        .class_object()
        .const_get::<_, RModule>("RustJSONSchema")
        .unwrap()
        .const_get("InvalidOptionsError")
        .unwrap();
    register_mark_object(ex); // avoid GC
    ex
});

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.define_module("RustJSONSchema")?;

    // RustJSONSchema::Validator
    let class = module.define_class("Validator", ruby.class_object())?;
    class.define_singleton_method("new", function!(Validator::new, -1))?;
    class.define_method("valid?", method!(Validator::is_valid, 1))?;
    class.define_method("validate", method!(Validator::validate, 1))?;
    class.define_method("options", method!(Validator::options, 0))?;

    // Ensure defined at load time
    Lazy::force(&JSON_PARSE_ERROR, ruby);
    Lazy::force(&SCHEMA_PARSE_ERROR, ruby);

    Ok(())
}
