use regex::Regex;
use crate::utils;

pub fn solve() {
    // split them into lines to read them
    let file = utils::read_file_as_str("day_1.txt");
    solve_part_one(file.clone());

}

/*
    Orders to solving:
        - Split apart the file contents string to make it more digestible
        - Iterate through every string in the list
        - Strip every non-digit from the string with a regex pattern
        - Collect numbers from the string
            - Shortcut it by checking the length of the string
            - Cut out the middle numbers
        - parse the numbers
        - add to a mutable total
        - print result
 */
fn solve_part_one(contents: String) {
    let file_contents = contents.split("\n");
    let mut total = 0;

    for entry in file_contents {
        let reg = Regex::new(r"[^0-9]").unwrap();
        let mut result = reg.replace_all(entry, "").to_string();

        let length = result.len();
        if length > 2 {
            result.replace_range(1..(length - 1), "");
        }

        let foo = result.trim().parse::<i32>().unwrap();
        println!("{}", result.trim());
        total += foo;

        println!("number: {result}");
    }
    println!("total: {total}");
    //utils::write_to_file("foo.txt", "bar");

}
