extern crate core;
mod day1;
mod util;

fn main(){
    util::time_all(|| {
        day1::solve()
    })
}