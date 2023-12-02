use regex::Regex;

mod utils;

fn main() {
    // split them into lines to read them
    let file = utils::read_file_as_str("day_1.txt");
    let file_contents = file.split("\n");
    // create a var to hold the grand total
    let mut total = 0;

    // Go through every line to find the calibration numbers
    for entry in file_contents {
        // split the whole line up
        let reg = Regex::new(r"/\D/g").unwrap();
        let mut result = reg.replace_all(entry, "").to_string();

        // we don't need to do excess calculation if the number is already there for us
        let length = result.len();

        if length > 2 {
            result.replace_range(1..(length - 2), "");
        }

        let foo = result.parse::<i32>().expect("Expected a digit");
        println!("{}", foo);
        total += foo;
        // total += result.parse::<i32>()?;

        println!("number: {result}");
    }
    println!("total: {total}");
    //utils::write_to_file("foo.txt", "bar");

}
