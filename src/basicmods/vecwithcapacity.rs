

//自动推倒vec元素类型
pub fn vecwithcapatict_main(){
    let mut s = Vec::with_capacity(2);
    s.push(100);
    s.push(234);
    println!("{:?}",s);
}