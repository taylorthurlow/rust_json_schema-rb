# frozen_string_literal: true

require_relative "rust_json_schema/version"
require_relative "rust_json_schema/rust_json_schema"

module RustJSONSchema
  class Error < StandardError; end
end
