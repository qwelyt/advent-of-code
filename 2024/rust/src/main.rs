extern crate core;
mod day1;
mod util;
mod day2;
mod day3;
mod day4;

fn main(){
    util::time_all(|| {
        day1::solve();
        day2::solve();
        day3::solve();
        day4::solve();
    })
}