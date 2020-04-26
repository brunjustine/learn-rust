//Training 1 _
//Generate the nth Fibonacci number.
//just.brun@hotmail.fr ~ Brun Justine

use std::io;

//f_1 et f_2
const F1:u64 = 1;
const F2:u64 = 1;

fn fibonacci (nth : i32) -> u64{
    //var result of the sum
    let mut res = 0;

    if nth<0 {
        panic!("{} is negative!",nth);
    } else if nth==0 {
        panic!("zero is not a right argument to fibonacci()!");
    } else if nth==1 || nth==2 {
        res =1;
    } else {
        
        let mut last = F1;
        let mut curr = F2;
        
        for _i in 2..nth {
            res = last + curr;
            last = curr;
            curr = res;
        }
    }
    res

}


fn main() {
    let mut nth = String::new();

    //read nth
    io::stdin().read_line(&mut nth)
            .expect("Failed to read line");

    //convert string to int
    let nth: i32 = nth.trim().parse()
            .expect("Failed to read number");

    let res = fibonacci(nth);
    println!("F_{} = {}",nth,res);
}
