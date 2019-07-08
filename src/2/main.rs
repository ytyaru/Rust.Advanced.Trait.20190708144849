/*
 * Rustの高度な機能（トレイト）。
 * CreatedAt: 2019-07-08
 */
use std::ops::Add;
struct Millimeters(u32);
struct Meters(u32);
impl Add<Meters> for Millimeters {
    type Output = Millimeters;
    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}
fn main() {}
