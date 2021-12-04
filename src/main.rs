pub mod lib;
pub use lib::*;

fn main() {
    // let d1_p1 = d1::run_part1();
    // println!("d1p1 result: {}", d1_p1);
    // let d1_p2 = d1::run_part2();
    // println!("d1p2 result: {}", d1_p2);
    // let d2_p1 = d2::run_part1();
    // println!("d2p1 result: {}", d2_p1);
    // let d2_p2 = d2::run_part2();
    // println!("d2p2 result: {}", d2_p2);
    let d3_p1 = d3::run_part1();
    println!("d3p1 result: {}", d3_p1);
    let d3_p2 = d3::run_part2().unwrap();
    println!("d3p2 result: {}", d3_p2);
}
