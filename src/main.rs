const cardinal_numbers: [&str; 12] = [
    "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth",
    "eleventh", "twelfth",
];
const ordinal_numbers: [&str; 12] = [
    "a partridge in a pear tree",
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

// const lyrics: &str = "On the twelfth day of Christmas,
// my true love gave to me
// Twelve drummers drumming,
// Eleven pipers piping,
// Ten lords a-leaping,
// Nine ladies dancing,
// Eight maids a-milking,
// Seven swans a-swimming,
// Six geese a-laying,
// Five golden rings,
// Four calling birds,
// Three French hens,
// Two turtle doves,
// And a partridge in a pear tree!";

fn main() {
    for (i, day) in cardinal_numbers.iter().enumerate() {
        println!(
            "On the {} day of Christmas,
my true love gave to me",
            day
        );
        for j in (0..=i).rev() {
            if j == 0 && i != 0 {
                print!("And ");
            }
            println!("{}", ordinal_numbers[j]);
        }
        println!();
    }
}
