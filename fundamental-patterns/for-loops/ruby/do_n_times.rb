#!/usr/bin/env ruby
#

def do_n_times(n, some_func)
  for i in 0..(n-1)
    $stdout.puts "action"
    $stdout.flush
  end

  method(some_func).call

  return
end
