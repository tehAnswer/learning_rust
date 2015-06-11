require 'ffi'

module CounterExamples
  extend FFI::Library
  ffi_lib 'target/release/libembed.dylib'
  attach_function :process, [], :void

  def self.pure_ruby
    start_time = Time.now
    threads = []
    10.times do
      threads << Thread.new() do
        count = 0

        5_000_000.times do
          count += 1
        end
      end
    end
    threads.each {|t| t.join }
    end_time = Time.now
    puts "  done in #{end_time - start_time}(s)!"
  end

  
  def self.rust_embed
    start_time = Time.now
    self.process
    end_time = Time.now
    puts "  done in #{end_time - start_time}(s)!"
  end

  def self.benchmark
    puts "CHAN CHAN CHAN...\n"

    puts "\nRuuuuuuuuby"
    puts "--------------"
    self.pure_ruby

    puts "\nRuuuuuuust\n"
    puts "--------------"
    self.rust_embed
    puts ""
  end
end

CounterExamples.benchmark
