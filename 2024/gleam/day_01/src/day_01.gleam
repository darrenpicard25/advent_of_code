import envoy
import gleam/int
import gleam/io
import gleam/list
import gleam/string

import gleam/result
import simplifile

pub fn main() {
  envoy.get("DATA_FILE_PATH")
  |> result.try(run_part_1)
  |> result.unwrap("Failed")
  |> string.append(to: "Part 1: ")
  |> io.println

  envoy.get("DATA_FILE_PATH")
  |> result.try(run_part_2)
  |> result.unwrap("Failed")
  |> string.append(to: "Part 2: ")
  |> io.println
}

fn parse_input(data: String) -> Result(#(List(Int), List(Int)), Nil) {
  data
  |> string.split(on: "\n")
  |> list.map(with: string.split(_, on: "  "))
  |> list.map(with: list.map(_, string.trim))
  |> list.map(with: list.map(_, int.base_parse(_, 10)))
  |> list.map(with: result.all)
  |> result.all
  |> result.map(with: list.map(_, fn(line) {
    case line {
      [first, second] -> {
        Ok(#(first, second))
      }
      _ -> Error(Nil)
    }
  }))
  |> result.try(result.all)
  |> result.map(with: list.fold(
    _,
    #([], []),
    fn(acc, value) {
      let first_list = list.prepend(this: value.0, to: acc.0)
      let second_list = list.prepend(this: value.1, to: acc.1)

      #(first_list, second_list)
    },
  ))
}

pub fn run_part_1(data_file_path: String) -> Result(String, Nil) {
  simplifile.read(data_file_path)
  |> result.map_error(fn(error) {
    io.print_error(simplifile.describe_error(error))
    Nil
  })
  |> result.map(parse_input)
  |> result.flatten
  |> result.map(fn(lists) {
    #(list.sort(lists.0, by: int.compare), list.sort(lists.1, by: int.compare))
  })
  |> result.map(fn(lists) { list.zip(lists.0, lists.1) })
  |> result.map(list.fold(
    _,
    0,
    fn(acc, value) { acc + int.absolute_value(value.0 - value.1) },
  ))
  |> result.map(int.to_string)
}

pub fn run_part_2(data_file_path: String) -> Result(String, Nil) {
  simplifile.read(data_file_path)
  |> result.map_error(fn(error) {
    io.print_error(simplifile.describe_error(error))
    Nil
  })
  |> result.map(parse_input)
  |> result.flatten
  |> result.map(fn(lists) {
    list.fold(over: lists.0, from: 0, with: fn(acc, number) {
      let number_times_present = list.count(lists.1, fn(n) { n == number })

      acc + number * number_times_present
    })
  })
  |> result.map(int.to_string)
}
