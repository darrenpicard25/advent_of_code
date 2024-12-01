import day_01
import gleeunit
import gleeunit/should

pub fn main() {
  gleeunit.main()
}

// gleeunit test functions end in `_test`
pub fn part_1_test() {
  day_01.run_part_1("data/sample.txt")
  |> should.equal(Ok("11"))
}

pub fn part_2_test() {
  day_01.run_part_2("data/sample.txt")
  |> should.equal(Ok("31"))
}
