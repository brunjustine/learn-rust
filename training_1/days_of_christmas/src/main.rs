//Training 1 _
//Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” 
//taking advantage of the repetition in the song.
//just.brun@hotmail.fr ~ Brun Justine


fn lyrics (day:&str) {
    match day {
        "first" => println!("a partridge in a pear tree"),
        "second" =>  println!("two turtle doves and"),
        "third" =>  println!("three French hens, "),
        "fourth" =>  println!("four calling birds, "),
        "fifth" =>  println!("five gold rings, "),
        "sixth" =>  println!("six geese a laying, "),
        "seventh" =>  println!("seven swans a swimming, "),
        "eighth" =>  println!("eight maids a milking, "),
        "ninth" =>  println!("nine ladies dancing, "),
        "tenth" =>  println!("ten lords a leaping, "),
        "eleventh" =>  println!("eleven pipers piping, "),
        "twelfth" =>  println!("twelve drummers drumming, "),
        _ => panic!{"Out of range"},
    }
}

fn main() {

    let days = ["first","second","third","fourth","fifth","sixth","seventh","eighth","ninth","tenth","eleventh","twelfth"];

    for i in 0..days.len() {
        // sentence of the day
        println!("On the {} days of Christmas my true love gave to me",days[i]);

        //gift of the day
        for j in (0..(i+1)).rev() {
            lyrics(days[j]);
        }
        println!("\n");
        
    }
}
