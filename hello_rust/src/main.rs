/* use std::io;
fn main() {
    let a = [1, 2, 3, 4, 5];
    println!("Please enter an array index");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
    let element = a[index];
    println!("The value of the element at index {index} is : {element}")
}
 */
/*  fn main(){
    println!("Hello world");
    another_function(5, 'h');
 }
 fn another_function(x : i32, label: char){
    println!("Another function is {x} {label}");
 } */
fn main(){
    let mut count =0;
    let _result = loop{
        count+=1;
        println!("The counter is {count}");
        if count==20{
            println!("It's over");
            break;
        }
    };
}