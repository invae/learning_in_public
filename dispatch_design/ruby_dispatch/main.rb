#!/usr/bin/env ruby

require './dispatcher'

ruby_argv_pain = [
    "self"
]
for item in ARGV
    ruby_argv_pain.push(item)
end
if ruby_argv_pain[1] != nil
    Dispatcher.entry( ruby_argv_pain[1] )
else
    $stdout.puts "provide at least one argv"
    $stdout.flush
end

