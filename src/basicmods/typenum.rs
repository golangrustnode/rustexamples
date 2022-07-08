use typenum::{Exp, Integer, Sum, N2, N4, P3, P4};
pub fn typenum_main() {
    assert_eq!(N4::to_i32(), -4);
    println!("{}", N4::to_i32());

    type X = Sum<P3, P4>;
    println!("{:?}", P3::to_i32());
    println!("{:?}", P4::to_i32());
    println!("{}", <X as Integer>::to_i32());
    assert_eq!(<X as Integer>::to_i32(), 7);

    type Y = Exp<N2, P3>;
    assert_eq!(<Y as Integer>::to_i32(), -8);
}
