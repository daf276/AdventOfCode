package aoc

import java.io.File

const val basic_path = "inputs/"

fun readInput(fileName: String): List<String> = File(fileName).readLines()

fun parseNums(input: List<String>): List<Int> = input.map { it.toInt() }