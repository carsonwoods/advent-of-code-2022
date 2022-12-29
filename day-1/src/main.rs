// Advent of Code 2022 -- Day 1
// Written by Carson Woods

fn main() {
    // read input from file contents
    let contents = std::fs::read_to_string("input.txt")
        .expect("Error: could not read input file");

    // split the contents of the file by blank lines (consecutive newline characters)
    let elves_cal: Vec<&str> = contents.split("\n\n").collect();

    // stores summed totals of calories will be stored
    let mut elves_cal_totals: Vec<u32> = vec![];

    // iterate through all elves snack calorie manifest
    for item in elves_cal.into_iter() {
        // split their calories into individual snacks
        let cal_seq: Vec<&str> = item.trim().split("\n").collect();

        // store single-elf-total
        let mut total = 0;

        // iterate through snacks adding calories to an elf's total
        for calorie in cal_seq.into_iter() {
            total = total + calorie.parse::<u32>().unwrap();
        }

        // add the total to the vector of totals
        elves_cal_totals.push(total);
    }

    // print the solution
    println!("The solution to part A:");
    println!("The largest total is: {:?}", elves_cal_totals.iter().max().unwrap());

    // reverse sort the vector of totals
    elves_cal_totals.sort_by(|a, b| b.cmp(a));

    // stores sum of 3 greatest totals
    let mut total = 0;

    // sums the 3 largest totals
    for x in 0..3 {
        total = total + elves_cal_totals[x];
    }

    // prints solutions
    println!("\nThe solution to part A:");
    println!("The total of the largest 3 calorie counts is: {}", total);

}
