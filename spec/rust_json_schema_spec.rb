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

    describe "#validate" do
      let(:schema) do
        JSON.generate(
          properties: {
            foo: {
              type: "string"
            },
            bar: {
              type: "number"
            },
            baz: {}
          },
          required: ["baz"]
        )
      end

      context "when the input is valid" do
        let(:input) do
          JSON.generate(
            foo: "foo",
            bar: 1,
            baz: "wadus"
          )
        end

        it "returns an empty array" do
          validator = RustJSONSchema::Validator.new(schema)
          expect(validator.validate(input)).to eq([])
        end
      end

      context "when the input is invalid" do
        let(:input) do
          JSON.generate(
            foo: 1,
            bar: "wadus"
          )
        end

        it "returns an array of errors" do
          validator = RustJSONSchema::Validator.new(schema)

          errors = validator.validate(input)

          expect(errors).to contain_exactly(
            'path "/bar": "wadus" is not of type "number"',
            'path "/foo": 1 is not of type "string"',
            'path "/": "baz" is a required property'
          )
        end
      end
    end
  end
end
