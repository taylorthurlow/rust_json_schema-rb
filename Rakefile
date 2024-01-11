# frozen_string_literal: true

require "bundler/gem_tasks"
require "rspec/core/rake_task"

RSpec::Core::RakeTask.new(:spec)

require "standard/rake"

require "rb_sys/extensiontask"

task build: :compile

RbSys::ExtensionTask.new("rust_json_schema") do |ext|
  ext.lib_dir = "lib/rust_json_schema"
end

task default: %i[compile spec standard]
