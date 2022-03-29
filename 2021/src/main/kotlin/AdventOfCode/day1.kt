package AdventOfCode

fun part1(input: List<Int>): Int = input.zipWithNext().count { (current, next) -> current < next }

fun part2(input: List<Int>): Int = input.windowed(4).count { measurements -> measurements[0] < measurements[3] }