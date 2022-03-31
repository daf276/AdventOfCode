package aoc

import io.kotest.core.spec.style.DescribeSpec
import io.kotest.matchers.shouldBe

class Tests : DescribeSpec({

    describe("Day1") {
        val input = readInput(basic_path + "day01")
        val parsed = parseNums(input)

        it("Part 1") {
            aoc.day1.part1(parsed) shouldBe 1688
        }

        it("Part 2") {
            aoc.day1.part2(parsed) shouldBe 1728
        }
    }

    describe("Day2") {
        val input = readInput(basic_path + "day02")
        val parsed = aoc.day2.parseInstructions(input)

        it("Part 1") {
            aoc.day2.part1(parsed) shouldBe 1648020
        }

        it("Part 2") {
            aoc.day2.part2(parsed) shouldBe 1759818555
        }
    }

})