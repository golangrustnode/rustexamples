
pub trait ChangeNumber<T> {
    fn double(&self,x:T)->T;
}

struct Point<T>{
    i: T,
    j: T,
}
/*
impl之后的T是声明T
ChangeNumber<T> trait当中的泛型
Point<T> 结构体泛型
*/
impl<T,U> ChangeNumber<T> for Point<U> {
    fn double(&self,x:T)->T{
        x
    }
}

pub fn maptraits_main(){
    /*
    let i:i32 = 3;
    println!("{}",i.double());
    let v:Vec<i32> = vec![1,3,4,5,6,9];
    //let v2 = v.into_iter().map(changeNumber::double);
    println!("{:?}",3.double());
    */
    let p = Point{
        i: 10,
        j: 30,
    };
    let v = p.double(200);
    println!("{}",v);
}