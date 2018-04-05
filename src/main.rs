extern crate ci_detective;

fn main() {
    if let Some(ci) = ci_detective::CI::eager() {
        println!("{:?}", ci);
    }
}
