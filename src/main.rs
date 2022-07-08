mod basicmods;

pub use crate::basicmods::from_into::frommain;
pub use crate::basicmods::typenum::typenum_main;
pub use crate::basicmods::maptraits::maptraits_main;
fn main() {
    //frommain();
    //typenum_main();
    maptraits_main();
}
