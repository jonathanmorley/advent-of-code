mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;

aoc_main::main! {
  year 2022;
  day01             => part1, part2;
  day02             => part1, part2;
  day03 :parse      => part1, part2;
  day04 :parse      => part1, part2;
  day05 :parse      => part1, part2;
  day06             => part1, part2_hashset, part2_loops;
  day07 :parse      => part1, part2;
  day08 :parse      => part1, part2;
  day09 :parse      => part1, part2;
  day10 :parse      => part1, part2;
  day11 :generate   => part1, part2;
}
