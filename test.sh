#!/usr/bin/env bash

assert() {
  expected="$1"
  input="$2"

  mkdir dist >/dev/null 2>&1
  ./target/debug/rust-rvcc "$input" >./dist/output.s || exit
  riscv64-unknown-elf-gcc -static -o ./dist/output ./dist/output.s
  spike --isa=rv64gc pk ./dist/output

  actual="$?"

  if [ "$actual" = "$expected" ]; then
    echo "$input => $actual"
  else
    echo "$input => $expected expected, but got $actual"
    exit 1
  fi
}

assert 0 0
assert 42 42

assert 34 '12-34+56'

assert 41 ' 12 + 34 - 5 '

echo OK
