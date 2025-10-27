fn main() {
    // 整数型
    let age: i32 = 25;
    println!("年齢: {}", age);
    
    // 浮動小数点型
    let height: f64 = 170.5;
    println!("身長: {}cm", height);
    
    // 真偽値型
    let is_student: bool = true;
    println!("学生ですか？: {}", is_student);
    
    // 文字型
    let grade: char = 'A';
    println!("成績: {}", grade);
    
    // 文字列型
    let name: String = String::from("太郎");
    println!("名前: {}", name);
}