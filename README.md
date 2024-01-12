# `rust_json_schema`

[![Gem Version](https://badge.fury.io/rb/rust_json_schema.svg)](https://rubygems.org/gems/rust_json_schema)

`rust_json_schema` is a Ruby wrapper gem for Rust's [jsonschema-rs crate](https://github.com/Stranger6667/jsonschema-rs).

This gem ships with precompiled binaries for Linux and macOS. Check the available gems on [Rubygems](https://rubygems.org/gems/rust_json_schema).

## Warning

I do not have any significant Rust programming experience, but this gem satisifies a need for a performant JSON Schema validation tool in Ruby land. While I intend to use this gem in a production environment, consider this code and library entirely experimental, at least until a 1.0 release, if it ever comes to that.

[rusty_json_schema](https://github.com/driv3r/rusty_json_schema) is a direct source of inspiration (and in some cases, literal copy and paste, like some fixtures/specs). Now that [bundler has explicit support for rust-backed Ruby gems](https://bundler.io/blog/2023/01/31/rust-gem-skeleton.html) as of early 2023, the Rust library code is a lot simpler that it previously needed to be, largely thanks to [magnus crate](https://github.com/matsadler/magnus) and the [rb-sys gem](https://github.com/oxidize-rb/rb-sys/tree/main/gem).

## Installation

Install the gem and add to the application's Gemfile by executing:

    $ bundle add rust_json_schema

If bundler is not being used to manage dependencies, install the gem by executing:

    $ gem install rust_json_schema

## Usage

```ruby
validator = RustJSONSchema::Validator.new(<<~JSON)
  {
    "properties": {
      "foo": { "type": "string" },
      "bar": { "type": "integer" },
      "baz": {}
    },
    "required": ["foo", "baz"]
  }
JSON

errors = validator.validate('{ "foo": 1, "bar": "wadus" }')
# => [
#   'path "/bar": "wadus" is not of type "number"',
#   'path "/foo": 1 is not of type "string"',
#   'path "/": "baz" is a required property'
# ]
```

### Errors

- All errors are subclasses of `RustJSONSchema::Error`.
- Calling `RustJSONSchema::Validator#new`, `#validate` or `#valid?` with a string which is not valid JSON will raise `RustJSONSchema::JSONParseError`.
- Calling `RustJSONSchema::Validator#new` with an invalid schema will raise `RustJSONSchema::SchemaParseError`.

## TODO

- Support passing options as `jsonschema-rs` does

## Development

TODO

## Contributing

Bug reports and pull requests are welcome.

## License

The gem is available as open source under the terms of the [MIT License](https://opensource.org/licenses/MIT).
