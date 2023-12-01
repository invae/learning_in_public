#!/bin/bash

proj_prefix="aoc_$1"
template="aoc_0"


ls $proj_prefix > /dev/null 2>/dev/null
if [ $? = 0 ]; then
  printf "$proj_prefix exists"
  exit 69
fi
cp -r $template $proj_prefix

