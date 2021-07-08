@echo off

set cflags=-Os -Wextra -Wall -Werror -Wno-unused-parameter
set ldflags=

echo compiling...
clang -o cassette.exe main.c %cflags%

