import kotlin.math.abs

fun main() {
  fun isSafe(record: Iterable<Int>): Boolean {
    return record
        .zipWithNext()
        .map { (a, b) -> b - a }
        .onEach { if (abs(it) !in 1..3) return@isSafe false }
        .zipWithNext()
        .all { (a, b) -> a * b >= 0 }
  }

  fun part1(input: List<String>): Int {
    var counter = 0
    for (line in input) {
      val record = line.split(" ").map { it.toInt() }
      if (isSafe(record)) counter++
    }
    return counter
  }

  fun part2(input: List<String>): Int {
    var counter = 0
    for (line in input) {
      val record = line.split(" ").map { it.toInt() }
      if (isSafe(record.drop(1))) counter++
      else {
        var badIndex: Int? = null
        run lit@{
          record
              .zipWithNext()
              .map { (a, b) -> b - a }
              .onEachIndexed { index, diff ->
                if (abs(diff) !in 1..3) {
                  badIndex = index + 1
                  return@lit
                }
              }
              .zipWithNext()
              .forEachIndexed() { index, (a, b) ->
                if (a * b < 0) {
                  badIndex = index + 1
                  return@lit
                }
              }
        }
        if (badIndex != null) {
          var rec = record.toMutableList()
          rec.removeAt(badIndex!!)
          if (isSafe(rec)) {
            counter++
          } else {
            rec = record.toMutableList()
            rec.removeAt(badIndex!! - 1)
            if (isSafe(rec)) {
              counter++
            }
          }
        }
      }
    }
    return counter
  }

  val input = readInput("input_02")
  part1(input).println()
  part2(input).println()
}
