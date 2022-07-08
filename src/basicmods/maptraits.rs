
pub trait ChangeNumber<T> {
    fn double(&self)->T;
}

struct Point<T>{
    i: T,
    j: T,
}

impl<T> ChangeNumber<T> for Point<T> where T:Copy {
    fn double(&self)->T{
        self.i
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
    let v = p.double();
    println!("{}",v);
}