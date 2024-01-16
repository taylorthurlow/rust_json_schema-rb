# frozen_string_literal: true

require "json"

RSpec.describe RustJSONSchema do
  it "has a version number" do
    expect(RustJSONSchema::VERSION).not_to be nil
  end

  describe RustJSONSchema::Validator do
    describe "#initialize" do
      describe ":draft option" do
        %i[draft4 draft6 draft7 draft201909 draft202012].each do |draft|
          it "accepts draft '#{draft}'" do
            schema = JSON.generate({})

            expect {
              RustJSONSchema::Validator.new(schema, draft: draft)
            }.not_to raise_exception
          end
        end

        it "raises an exception when the draft is not supported" do
          schema = JSON.generate({})

          expect {
            RustJSONSchema::Validator.new(schema, draft: "foo")
          }.to raise_exception "invalid draft: 'foo'"
        end
      end

      describe ":with_meta_schemas option" do
        it "does not raise an exception when provided" do
          schema = JSON.generate({})
          expect {
            RustJSONSchema::Validator.new(schema, with_meta_schemas: true)
          }.not_to raise_exception
        end
      end
    end

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

          errors = validator.validate(input)

          expect(errors).to eq([])
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

      context "when the input is invalid JSON" do
        it "raises RustJSONSchema::JSONParseError" do
          validator = RustJSONSchema::Validator.new(schema)

          expect {
            validator.validate("not valid json")
          }.to raise_exception(RustJSONSchema::JSONParseError)
        end
      end
    end

    describe "#valid?" do
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

        it "returns true" do
          validator = RustJSONSchema::Validator.new(schema)

          result = validator.valid?(input)

          expect(result).to be true
        end
      end

      context "when the input is invalid" do
        let(:input) do
          JSON.generate(
            foo: 1,
            bar: "wadus"
          )
        end

        it "returns false" do
          validator = RustJSONSchema::Validator.new(schema)

          result = validator.valid?(input)

          expect(result).to be false
        end
      end

      context "when the input is invalid JSON" do
        it "raises RustJSONSchema::JSONParseError" do
          validator = RustJSONSchema::Validator.new(schema)

          expect {
            validator.valid?("not valid json")
          }.to raise_exception(RustJSONSchema::JSONParseError)
        end
      end
    end

    describe "#options" do
      it "has some default options" do
        schema = JSON.generate({})

        validator = RustJSONSchema::Validator.new(schema)

        expect(validator.options).to eq(
          draft: :draft7,
          with_meta_schemas: false
        )
      end

      it "returns a hash with the options" do
        schema = JSON.generate({})

        validator = RustJSONSchema::Validator.new(
          schema,
          draft: :draft4,
          with_meta_schemas: true
        )

        expect(validator.options).to eq(
          draft: :draft4,
          with_meta_schemas: true
        )
      end
    end
  end
end
