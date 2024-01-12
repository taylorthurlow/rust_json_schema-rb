# frozen_string_literal: true

require "bundler/gem_tasks"
require "rspec/core/rake_task"

RSpec::Core::RakeTask.new(:spec)

require "standard/rake"

require "rb_sys/extensiontask"

task build: :compile

rubies = ["3.3.0", "3.2.0", "3.1.0", "3.0.0"]
ENV["RUBY_CC_VERSION"] = rubies.join(":")

spec = Gem::Specification.load("rust_json_schema.gemspec")

RbSys::ExtensionTask.new("rust_json_schema", spec) do |ext|
  ext.lib_dir = "lib/rust_json_schema"

  ext.cross_compile = true
end

task default: %i[compile spec standard]
