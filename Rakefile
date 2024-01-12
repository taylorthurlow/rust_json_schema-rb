# frozen_string_literal: true

require "bundler/gem_tasks"
require "rspec/core/rake_task"

RSpec::Core::RakeTask.new(:spec)

require "standard/rake"

require "rb_sys/extensiontask"

task build: :compile

RbSys::ExtensionTask.new("rust_json_schema") do |ext|
  ext.lib_dir = "lib/rust_json_schema"

  ext.cross_compile = true
  ext.cross_platform = %w[x86-mingw32 x64-mingw-ucrt x64-mingw32 x86-linux x86_64-linux x86_64-darwin arm64-darwin]
end

task default: %i[compile spec standard]
