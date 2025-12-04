extern crate core;
mod util;
mod day1;
mod day2;
mod day3;

fn main(){
    util::time_all(|| {
        day1::solve();
        day2::solve();
        day3::solve();
    })
}