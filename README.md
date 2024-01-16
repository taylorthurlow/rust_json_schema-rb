# `rust_json_schema`

[![Gem Version](https://badge.fury.io/rb/rust_json_schema.svg)](https://rubygems.org/gems/rust_json_schema)

`rust_json_schema` is a Ruby wrapper gem for Rust's [jsonschema-rs crate](https://github.com/Stranger6667/jsonschema-rs).

This gem ships with precompiled binaries for Linux and macOS. Check the available gems on [Rubygems](https://rubygems.org/gems/rust_json_schema). Precompiled binaries do not exist for non-standard rubies like JRuby or TruffleRuby, nor do they exist for Windows. I will review and accept PRs if you would like to work on adding these build targets.

## Warning

I do not have any significant Rust programming experience, but this gem satisifies a need for a performant JSON Schema validation tool in Ruby land. While I intend to use this gem in a production environment, consider this code and library entirely experimental, at least until a 1.0 release, if it ever comes to that.

[rusty_json_schema](https://github.com/driv3r/rusty_json_schema) is a direct source of inspiration (and in some cases, literal copy and paste, like some fixtures/specs). Now that [bundler has explicit support for rust-backed Ruby gems](https://bundler.io/blog/2023/01/31/rust-gem-skeleton.html) as of early 2023, the Rust library code is a lot simpler that it previously needed to be, largely thanks to [magnus crate](https://github.com/matsadler/magnus) and the [rb-sys gem](https://github.com/oxidize-rb/rb-sys/tree/main/gem), and by extension, the [oxidize-rb team](https://github.com/oxidize-rb).

## Installation

Install the gem and add to the application's Gemfile by executing:

    $ bundle add rust_json_schema

If bundler is not being used to manage dependencies, install the gem by executing:

    $ gem install rust_json_schema

## Usage

```ruby
schema = <<~JSON
  {
    "properties": {
      "foo": { "type": "string" },
      "bar": { "type": "integer" },
      "baz": {}
    },
    "required": ["foo", "baz"]
  }
JSON

validator = RustJSONSchema::Validator.new(
  schema,
  draft: :draft7,
  with_meta_schemas: false
)

errors = validator.validate('{ "foo": 1, "bar": "wadus" }')
# => [
#   'path "/bar": "wadus" is not of type "number"',
#   'path "/foo": 1 is not of type "string"',
#   'path "/": "baz" is a required property'
# ]
```

### Options

- `:draft` - Select the JSON schema draft number to use. Valid options are `draft4`, `draft6`, `draft7`, `draft201909`, and `draft202012`. Supported drafts are entirely determined by the `jsonschema` crate. The default draft is also determined by the crate. If new versions of the crate support additional draft versions, a code change in this gem will be required. I'm open to PRs to solve this problem - I don't know enough Rust to tell if it's easily done. *Both `draft201909` and `draft202012` are reported to have "some keywords not implemented", so use them at your own risk.*
- `:with_meta_schemas` - See [docs.rs/jsonschema CompilationOptions with_meta_schemas](https://docs.rs/jsonschema/0.17.1/jsonschema/struct.CompilationOptions.html#method.with_meta_schemas). `false` by default.

Any additional options provided by the `jsonschema` crate are options I do not understand or may not make sense to implement in a wrapper library such as this.

### Errors

- All errors are subclasses of `RustJSONSchema::Error`.
- Calling `RustJSONSchema::Validator#new`, `#validate` or `#valid?` with a string which is not valid JSON will raise `RustJSONSchema::JSONParseError`.
- Calling `RustJSONSchema::Validator#new` with an invalid schema will raise `RustJSONSchema::SchemaParseError`.
- Calling `RustJSONSchema::Validator#new` with an invalid draft version value will raise `RustJSONSchema::InvalidOptionsError`.

## Development

TODO

## Contributing

Bug reports and pull requests are welcome.

## License

The gem is available as open source under the terms of the [MIT License](https://opensource.org/licenses/MIT).
