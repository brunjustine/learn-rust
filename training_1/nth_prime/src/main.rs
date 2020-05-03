
fn is_prime(n:u32)-> bool {
    let mut m = 2;
    while m<n && n%m != 0{
        m+=1;
    }
    m==n
}

pub fn nth(n: u32) -> u32 {
    let mut i = 2;
    let mut m =0;
    while m < n {
	i+=1;
        if is_prime(i) {
            println!("i : {}" ,i);
            m +=1;
		println!("m : {}" ,m);
        }
    }
	 println!("i : {}" ,i);
    i
}

fn main() {
	 println!("premier nth : {}",nth(0));
	 println!("premier nth : {}",nth(1));
	 println!("premier nth : {}",nth(2));
	 println!("premier nth : {}",nth(3));
	 println!("premier nth : {}",nth(4));
	 println!("premier nth : {}",nth(5));
	 println!("premier nth : {}",nth(6));
}
