#!/usr/bin/env ruby
#
require './do_n_times'

def x
  $stdout.puts "my func"
  $stdout.flush
end



do_n_times(2, :x)


my_array = Array[
  0,
  1,
  2,
  3,
  4,
  5,
  6,
  7,
  8,
  9,
]


for item in my_array
  $stdout.puts item * 2
  $stdout.flush
end


my_dict = { 
 "one"   => "eins",
 "two"   => "zwei",
 "three" => "drei",
}

my_dict.each do |k, v|
  $stdout.puts my_dict[k] 
  #$stdout.puts v 
  $stdout.flush
end

if my_dict['one'] == nil
  $stdout.puts "not implemented"
  $stdout.flush
end
