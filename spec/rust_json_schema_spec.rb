# frozen_string_literal: true

require "json"

RSpec.describe RustJSONSchema do
  it "has a version number" do
    expect(RustJSONSchema::VERSION).not_to be nil
  end

  describe RustJSONSchema::Validator do
    it "raises an error when the input schema is not valid JSON" do
      schema = "not valid json"

      expect {
        RustJSONSchema::Validator.new(schema)
      }.to raise_exception(RustJSONSchema::JSONParseError)
    end

    it "raises an error when the input schema is valid JSON but not a valid schema" do
      schema = '{"type": "invalidtype"}'

      expect {
        RustJSONSchema::Validator.new(schema)
      }.to raise_exception(RustJSONSchema::SchemaParseError)
    end
  end
end
