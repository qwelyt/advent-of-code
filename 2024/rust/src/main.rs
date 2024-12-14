extern crate core;
mod day1;
mod util;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;

fn main(){
    util::time_all(|| {
        day1::solve();
        day2::solve();
        day3::solve();
        day4::solve();
        day5::solve();
        day6::solve();
        day7::solve();
        day8::solve();
        day9::solve();
        day10::solve();
        day11::solve();
        day12::solve();
        day13::solve();
    })
}