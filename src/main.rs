mod parsing;

static mut SYMBOLARRAY: [[bool; 140]; 140] = [[false; 140]; 140];

fn canfill(x: usize, y: usize) -> bool {
    return x >= 0 && x < 140 && y >= 0 && y < 140;
        
}

fn filladjacent(x: usize, y: usize) {
    unsafe {
        if canfill(x-1, y-1) {
            SYMBOLARRAY[x-1][y-1] = true;
        }
        if canfill(x-1, y) {
            SYMBOLARRAY[x-1][y] = true;
        } 
        if canfill(x-1, y+1) {
            SYMBOLARRAY[x-1][y+1] = true;
        }
        if canfill(x, y-1) {
            SYMBOLARRAY[x][y-1] = true;
        }
        if canfill(x, y) {
            SYMBOLARRAY[x][y] = true;
        } 
        if canfill(x, y+1) {
            SYMBOLARRAY[x][y+1] = true;
        }
        if canfill(x+1, y-1) {
            SYMBOLARRAY[x+1][y-1] = true;
        }
        if canfill(x+1, y) {
            SYMBOLARRAY[x+1][y] = true;
        } 
        if canfill(x+1, y+1) {
            SYMBOLARRAY[x+1][y+1] = true;
        } 
    }
}



fn main() {    
    let lines: Vec<String> = parsing::parse_lines_of_strings(String::from("./src/engine.txt"));

    let mut total = 0;
    for (pos, line) in lines.iter().enumerate() {
        for l in line.char_indices() {
            if l.1 != '.' && !l.1.is_ascii_digit() {
                filladjacent(l.0, pos);
            }
        }
    }
    
    for (pos, line) in lines.iter().enumerate() {
        let mut on_number = false;
        let mut found_symbol = false;
        let mut subtotal = 0;
        for l in line.char_indices() {
            unsafe {
                // Potential scenarios
                // On a number that is valid                (onnumber = true, found_symbol = true, is_digit = true)     Add to subtotal
                // Just finished a number that is valid     (onnumber = true, found_symbol = true, is_digit = false)    Add to TOTAL, onnumber = false, foundsymbol = false
                // On a number that is not valid            (on_number = true, found_symbol = false, is_digit = true)   Add to subtotal
                // Just finished a number that wasnt valid  (on_number = true, found_symbol = false, is_digit = false)  Subtotal = 0, onnumber = false 
                // Just started a number that is valid      (on_number = false, found_symbol = true, is_digit = true)   Onnumber = true, add to subtotal
                // Not on a number and its but its valid    (on_number = false, found_symbol = true, is_digit = false)  Nothing
                // started a number not sure if its valid   (on_number = false, found_symbol = false, is_digit = true)  Add to subtotal, onnumber = true
                //                                          (on_number = false, found_symbol = false, is_digit = false) nothing
                
                if !found_symbol {
                    if SYMBOLARRAY[l.0][pos] && l.1.is_ascii_digit() {
                        found_symbol = true;
                    }
                }

                if l.1.is_ascii_digit() {
                    subtotal = subtotal * 10 + l.1.to_digit(10).unwrap();
                    on_number = true;
                }
                else {
                    if on_number && found_symbol {
                        total += subtotal;
                        if pos == 0 {
                            println!("{}", subtotal);
                        }
                        
                        subtotal = 0;
                        on_number = false;
                        found_symbol = false;
                    }
                    else if on_number && !found_symbol {
                        subtotal = 0;
                        on_number = false;
                    }
                }

                if l.0 == 139 && on_number && found_symbol {
                    total += subtotal;
                    //println!("{}", subtotal);
                }                    
            }
        }
    }
    println!("{}", total);

    
}