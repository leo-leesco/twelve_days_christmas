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
