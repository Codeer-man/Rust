// const PI :f64 = 3.1415;

fn main() {
    //     println!("hello world");
    //     let mut add :u32 = 1;
    //     add += 1;

    //     println!("{add}");
    //     println!("{PI}");

    //!     variable shadowning

    //     let n =5;
    //     {
    //         let n = 10;
    //         println!("Inner n is {n}");
    //     }
    //     println!("Outer n is {n}");

    //     let space = "     ";
    //     let space = space.len();

    //     println!("the space is{space}");

    //! convet the data
    // let user_input = "100";
    // let convert :u32 = user_input.parse().expect("scould not parse");
    // println!("convert {}",convert +100);

    //! integers
    let _n1: i8 = -10; // signed integer -128 to127
    let _n2: u8 = 100; // unsigned integer 0 to 255

    // println!("{}", isize::MAX);
    // println!("{}", isize::MIN);

    //? floater
    let f1 : f64 = 0.1;
    let f2 : f64= 0.2;
    let sum :f64 = f1 + f2;

    println!("sum :{}",sum);
    
    //? boolean 
    println!("sum :{}",sum == 0.3);
    let has_money: i32 = 500;

    if has_money <= 0{
        println!("You are broke")
    }else {
        println!("You are lucky")
    }

    // character 
    let word : char = 'm';
    let omega : char = 'Ω';
    let pie : char = 'π';

    println!("word:{word} \n omega: {omega} \n pie:{pie}")
}
