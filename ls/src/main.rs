
const PI :f64 = 3.1415;

fn main(){
    println!("hello world");
    let mut add :u32 = 1;
    add += 1;

    println!("{add}");
    println!("{PI}");

//    variable shadowning  

    let n =5;
    {
        let n = 10;
        println!("Inner n is {n}");
    }
    println!("Outer n is {n}");

    let space = "     ";
    let space = space.len();

    println!("the space is{space}");

}