use std::io;
use std::cmp::Ordering;

// ability to accept uder input
use rand::Rng;



fn main(){
    loop{

println!("guess a number");

let secret_number= rand::thread_rng().gen_range(1..=100);
println!("plear input your number");
//println!("secret number {secret_number}");

// we have decalered a  variable which creates a empty string 
let mut guess =String::new();

io::stdin().read_line(&mut guess).expect("failed to read value");


//let guess : u32 = guess.trim().parse().expect("it is not integer");

let guess: u32 =match guess.trim().parse(){
    Ok (num)=> num,
    Err(_)=> continue,



};
// let x =4;
// let y= 5;
//println!("x={x},y+5={y}+5");
println!("you guessed :{guess}"); 



match guess.cmp(&secret_number) {

Ordering::Less => println!("too small"),
Ordering::Greater => println!("too big"),
Ordering::Equal =>{



 println!("equal");
 break;

}
}


}

}
