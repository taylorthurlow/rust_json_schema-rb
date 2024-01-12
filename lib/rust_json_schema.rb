# frozen_string_literal: true

require_relative "rust_json_schema/version"

# Tries to require the extension for the given Ruby version first
begin
  RUBY_VERSION =~ /(\d+\.\d+)/
  require "rust_json_schema/#{Regexp.last_match(1)}/rust_json_schema"
rescue LoadError
  require_relative "rust_json_schema/rust_json_schema"
end

module RustJSONSchema
  class Error < StandardError; end

  class JSONParseError < Error; end

  class SchemaParseError < Error; end
end
