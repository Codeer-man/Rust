use rand::Rng;
use std::io;
use std::cmp::Ordering;


fn main(){
    println!("Welcome to Guess The Number Game");

    
    let random_number : u32 = rand::rng().random_range(1..=100);
    // let random_number : u32 = rand::thread_rng().gen_range(1..=100)'

    

    println!("{}",random_number);
    println!("Enter a random number");
    let mut tries: u32 = 0;
    loop{
        let mut guess:String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to readline");

        let guess :u32 =guess.trim().parse().expect("Enter a valid number");
        println!("Your entered number is :{}",guess);

        match guess.cmp(&random_number){
            Ordering::Greater => {println!("Too big!!!");
            tries += 1 ;
        },
            Ordering::Less =>{ println!("Too small!!");
            tries +=1
        },
            Ordering::Equal => {
                println!("You guessed it correct");
                println!("The Number was: {}",random_number);
                tries += 1 ;
                break;
            }
        }


    }
    println!("You completed in {} trys ",tries)
}