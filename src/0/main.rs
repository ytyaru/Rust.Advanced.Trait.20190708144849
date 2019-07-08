/*
 * Rustの高度な機能（トレイト）。
 * CreatedAt: 2019-07-08
 */
fn main() {
    println!("Hello Rust !!");
}
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
struct Counter { count: u32 }
impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> { None }
}
