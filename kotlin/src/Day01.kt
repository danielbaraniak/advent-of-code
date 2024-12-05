import kotlin.math.abs

fun main() {
    fun part1(input: List<String>): Int {
        val separator = Regex("\\s+")
        val listLeft = mutableListOf<Int>()
        val listRight = mutableListOf<Int>()

        for (line in input) {
            val (nl, nr) = line.split(separator, 2).map { it.toInt() }
            listLeft.binarySearch(nl).let { listLeft.add(if (it < 0) -it - 1 else it, nl) }
            listRight.binarySearch(nr).let { listRight.add(if (it < 0) -it - 1 else it, nr) }
        }

        return listLeft.zip(listRight).sumOf { (l, r) -> abs(l - r) }
    }

    fun part2(input: List<String>): Int {
        val separator = Regex("\\s+")
        val listLeft = mutableListOf<Int>()
        val listRight = mutableMapOf<Int,Int>().withDefault { 0 }

        for (line in input) {
            val (nl, nr) = line.split(separator, 2).map { it.toInt() }
            listLeft.add(nl)
            listRight[nr] = listRight.getValue(nr) + 1
        }

        return listLeft.sumOf { it * listRight.getValue(it) }
    }

    val input = readInput("input_01")
    part1(input).println()
    part2(input).println()
}
