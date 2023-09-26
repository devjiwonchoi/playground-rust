fn main() {
    let nth_days = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eight",
        "ninth",
        "tenth",
        "eleventh",
        "twelfth",
    ];
    let lyrics = [
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    let mut i = 0;

    while i <= lyrics.len() {
        let mut j = 0;
        println!("[Verse {}]", i + 1);
        println!("On the {} day of Christmas my true love sent to me", nth_days[i]);
        while j < i {
            if i - j == 1 {
                println!("{}, and", lyrics[j]);
            } else {
                println!("{}", lyrics[j]);
            }
            j += 1
        }
        println!("A partridge in a pear tree\n");
        i += 1
    }

    println!("- THE END -\n");
}
