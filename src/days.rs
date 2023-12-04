fn day6() {
    let line: String = parsing::parse_lines_of_strings(String::from("./src/code.txt")).pop().unwrap();

    let mut index: i32 = 0;

    let mut map:HashSet<char> = HashSet::new();
    let mut found = false;

    for i in 0..line.len() {
        let tempstring = line[i..i+14].to_string();
        println!("{}", tempstring);
        map.clear();
        let mut count = 0;
        for t in tempstring.chars() {
            if map.insert(t) {
                count += 1;
            }
        }
        if count == 14 {
            index = (i as i32) + 14;
            break;
        }
    }
    
    println!("{}", index);
}