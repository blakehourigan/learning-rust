// in this program we print the twelve days of christmas lyrics, taking advantage of the repetition
// in the song!

fn main() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let gifts = [
        "A partridge in a pear tree.",
        "Two turtle doves,",
        "Three French hens,",
        "Four calling birds,",
        "Five gold rings,",
        "Six geese a-laying,",
        "Seven swans a-swimming,",
        "Eight maids a-milking,",
        "Nine ladies dancing,",
        "Ten lords a-leaping,",
        "Eleven pipers piping,",
        "Twelve drummers drumming,",
    ];

    for i in 0..=11 {
        println!(
            "On the {0} day of Christmas,\nmy true love sent to me",
            days[i]
        );

        for j in (0..=i).rev() {
            println!("{}", gifts[j]);
        }
        println!();
    }
}
