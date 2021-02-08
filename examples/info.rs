extern crate scbbindings;

fn main() {
    let scb = scbbindings::SCB::new("en");
    println!("{}", scb.api_base());
    let resp = scb.info();
    println!("{:#?}", resp);
}
