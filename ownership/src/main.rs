fn main() {
    let mut s = String::from("hello");
    
    let r1 = &mut s;
   // let r2 = &mut s;  // エラー！可変借用は同時に1つだけ
    
   // println!("{}, {}", r1, r2);
    println!("{}", r1);
}