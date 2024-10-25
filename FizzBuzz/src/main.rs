//loop 1-100
//check div by 3 and put Fizz
//check div by 5 and put Buzz
//if both then FizzBuzz

fn main() {
    for i in 1..=100 {
        if i % 3 == 0 && i % 5 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz")
        } else if i % 5 == 0 {
            println!("Buzz")
        } else {
            println!("{i}")
        }
    }
}
