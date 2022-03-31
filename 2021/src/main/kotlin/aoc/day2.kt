package aoc.day2

sealed class Instruction {
    data class FORWARD(val value: Int) : Instruction()
    data class DOWN(val value: Int) : Instruction()
    data class UP(val value: Int) : Instruction()
}


fun parseInstructions(input: List<String>): List<Instruction> =
    input.map { it.split(" ") }.map { (instruction, value) ->
        when (instruction) {
            "forward" -> Instruction.FORWARD(value.toInt())
            "down" -> Instruction.DOWN(value.toInt())
            "up" -> Instruction.UP(value.toInt())
            else -> throw Exception("This when is exhaustive")
        }
    }

fun part1(input: List<Instruction>): Int {
    val (pos, depth) = input.fold(Pair(0, 0)) { sum, add ->
        when (add) {
            is Instruction.FORWARD -> sum.copy(first = sum.first + add.value)
            is Instruction.DOWN -> sum.copy(second = sum.second + add.value)
            is Instruction.UP -> sum.copy(second = sum.second - add.value)
        }
    }

    return pos * depth
}

fun part2(input: List<Instruction>): Int {
    val (pos, depth, aim) = input.fold(Triple(0, 0, 0)) { sum, add ->
        when (add) {
            is Instruction.FORWARD -> sum.copy(
                first = sum.first + add.value,
                second = sum.second + sum.third * add.value
            )
            is Instruction.DOWN -> sum.copy(third = sum.third + add.value)
            is Instruction.UP -> sum.copy(third = sum.third - add.value)
        }
    }

    return pos * depth
}
