fn day2() {
    let lines: Vec<String> = parsing::parse_lines_of_strings(String::from("./src/cubes.txt"));

    let red: i32 = 12;
    let green: i32 = 13;
    let blue: i32 = 14;

    let mut total: i32 = 0;
    for line in lines {
        let temp: Vec<&str> = line.split(|x| x == ':' || x == ',' || x == ';' || x == ' ').collect();
        let worth: i32 = temp.index(1).parse().unwrap();

        let mut last: i32 = 0;
        let mut elim: bool = false;

        let mut minred = 0;
        let mut mingreen = 0;
        let mut minblue = 0;


        for t in temp.clone() {
            match t {
                "green" => if last > mingreen {
                    mingreen = last;
                },
                "red" => if last > minred {
                    minred = last;
                },
                "blue" => if last > minblue {
                    minblue = last;
                },
                _ => (),
            }

            last = match t.parse() {
                Ok(num) => num,
                Err(_num) => 0, 
            };

        }
        
        let sum: i32 = minblue * mingreen * minred;
        total += sum;
    }
    println!("{}", total);
}