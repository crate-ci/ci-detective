extern crate ci_detective;

fn main() {
    let circle = ci_detective::Circle::eager();
    println!("{:?}", circle)
}
