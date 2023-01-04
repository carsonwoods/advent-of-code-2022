// Advent of Code 2022 -- Day 3
// Written by Carson Woods

fn main() {
    let mut priorities = std::collections::HashMap::new();
    let mut priority_value = 1;

    // iterate through ascii values of chars to iterate through
    // entire lowercase alphabet
    for c in b'a'..=b'z' {
        priorities.insert(c as char, priority_value);
        priority_value = priority_value + 1;
    }

    // iterate through ascii values of chars to iterate through
    // entire uppercase alphabet
    for c in b'A'..=b'Z' {
        priorities.insert(c as char, priority_value);
        priority_value = priority_value + 1;
    }

    part_a(&priorities);
    part_b(&priorities)
}

fn part_a(priorities: &std::collections::HashMap<char, u32>) {
    // store priorities total
    let mut priorities_total = 0;

    // read input from file contents
    let contents = std::fs::read_to_string("input.txt").expect("Error: could not read input file");

    // splits input into vector of lines
    let rucksacks: Vec<&str> = contents.trim().split("\n").collect();

    for rucksack in rucksacks.into_iter() {
        // skip trailing new line that my editor appends to files
        // (plus it skips any invalid lines which only have a single item)
        let rucksack_len = rucksack.len();

        // if rucksack length is invalid, skip iteration
        if rucksack_len <= 1 {
            continue;
        }

        // split line into char vector
        let compartments: Vec<char> = rucksack.chars().collect();

        // slice the vector into chunks of len/2 size
        let mut slice_iter = compartments.chunks(rucksack_len / 2);

        // get the two slices as the respective compartments
        let compartment_one = slice_iter.next().unwrap();
        let compartment_two = slice_iter.next().unwrap();

        // stores list of items in both compartments
        let mut items_in_both: Vec<char> = vec![];

        // for each item in compartment one
        // adds that item to list to sum if found in compartment 2
        for c in compartment_one.into_iter() {
            if compartment_two.contains(c) {
                items_in_both.push(*c);
            }
        }

        // sorts duplicates so that dedup works correction
        items_in_both.sort();
        // ensures uniqueness of each item
        items_in_both.dedup();

        // finds total of priorities of items in both compartments
        for c in items_in_both.into_iter() {
            priorities_total = priorities_total + priorities.get(&c).unwrap();
        }
    }

    println!("The answer to part A: {}", priorities_total);
}

fn part_b(priorities: &std::collections::HashMap<char, u32>) {
    // store priorities total
    let mut priorities_total = 0;

    // read input from file contents
    let contents = std::fs::read_to_string("input.txt").expect("Error: could not read input file");

    // splits input into vector of lines
    let rucksacks: Vec<&str> = contents.trim().split("\n").collect();

    // iterate through groups
    for slice in rucksacks.chunks(3) {
        // iterate through items in rucksack of first elf in each chunk
        for c in slice[0].chars() {
            // check if badge candidate is found in other elves rucksacks
            if slice[1].contains(c) && slice[2].contains(c) {
                // badge found, add total to value
                priorities_total = priorities_total + priorities.get(&c).unwrap();
                // badge found, no need to continue searching
                break;
            }
        }
    }

    // print answer
    println!("The answer to part B: {}", priorities_total);
}
