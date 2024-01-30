//Copied from a youtube video, a bit difficult for beginners, but as long as i get the workings approximately, i should be able to move on.

fn main() {
    let day_lyrics = [
        "1st",
        "2nd",
        "3rd",
        "4th",
        "5th",
        "6th",
        "7th",
        "8th",
        "9th",
        "10th",
        "11th",
        "12th",
    ];
    
    let gift_lyrics = [
        "A partridge in a pear tree",
        "2 turtle doves",
        "3 French hens",
        "4 calling birds",
        "5 golden rings",
        "6 geese a-laying",
        "7 swans a-swimming",
        "8 maids a-milking",
        "9 ladies dancing",
        "10 lords a-leaping",
        "11 pipers piping",
        "12 drummers drumming",
    ];

    for day in 0..12 {
        christmas(day, day_lyrics, gift_lyrics);
        println!("\n");
    }
}

fn christmas(n: i32, day_lyrics: [&str;12], gift_lyrics: [&str;12]) {
    for day in (0..n+1).rev(){
        if day == n {
            println!("On the {} day of Christmas my true love sent to me", day_lyrics[day as usize]);
            println!("{}", gift_lyrics[day as usize]);
        } else if day == 0 {
            println!("And {}", gift_lyrics[day as usize].to_lowercase());
        } else {
            println!("{}", gift_lyrics[day as usize]);
        }
    }
}