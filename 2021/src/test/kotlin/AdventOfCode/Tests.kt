package AdventOfCode

import io.kotest.core.spec.style.DescribeSpec
import io.kotest.matchers.shouldBe

class Tests : DescribeSpec({

    describe("Day1") {
        val input = readInput(basic_path + "day01")
        val parsed = parseNums(input)

        it("Part 1") {
            part1(parsed) shouldBe 1688
        }

        it("Part 2") {
            part2(parsed) shouldBe 1728
        }
    }

})