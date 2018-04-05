extern crate ci_detective;

fn main() {
    if let Some(ci) = ci_detective::CI::from_env() {
        println!("{:?}", ci);
    }
}
