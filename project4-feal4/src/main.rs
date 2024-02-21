use std::fs;

fn f(input: u32) -> u32 {
    // Unpack bytes from a 32-bit word
    let x0 = (input >> 24) as u8;
    let x1 = (input >> 16) as u8;
    let x2 = (input >> 8) as u8;
    let x3 = input as u8;

    // Define helper functions g0 and g1 inline
    let g0 = |a: u8, b: u8| -> u8 {
        let result = (a as u32).wrapping_add(b as u32) % 256;
        let shifted_result = ((result << 2) | (result >> 6)) as u8;
        shifted_result
    };
    
    let g1 = |a: u8, b: u8| -> u8 {
        let result = ((a as u32).wrapping_add(b as u32) + 1) % 256;
        let shifted_result = ((result << 2) | (result >> 6)) as u8;
        shifted_result
    };

    // Perform calculations inline
    let y0 = g0(x0, x1);
    let y1 = g1(x0 ^ x1, x2 ^ x3);
    let y2 = g0(y1, x2 ^ x3);
    let y3 = g1(y2, x3);
    
    // Pack bytes into a 32-bit word
    let result: u32 = u32::from(y0) << 24 | u32::from(y1) << 16 | u32::from(y2) << 8 | u32::from(y3);
    //println!("Result: {} Input: {}", result, input);
    result
}

fn main() {
    let mut found_keys = Vec::new();

    if let Ok(file_contents) = fs::read_to_string("./know.txt") {
        let mut data = Vec::new();
        for line in file_contents.lines() {
            if line.is_empty() {
                continue;
            } else {
                let cleaned_line = line
                    .replace("Plaintext=  ", "")
                    .replace("Ciphertext= ", "")
                    .trim()
                    .to_lowercase();

                data.push(cleaned_line);
            }
        }
        let key_range =u32::MAX;
        for key in 0..=key_range {
            if key % 1000000 == 0 {
                println!("Keys: {}/{}", key, key_range);
            }
            let mut sum_zeros = 0;
            let mut sum_ones = 0;
            for text in data.chunks(2) {
                if let [plaintext, ciphertext] = text {
                    let l0 = match u32::from_str_radix(&plaintext[0..8], 16){
                        Ok(value) => value,
                        Err(e) => {
                            println!("Error parsing hexadecimal: {:?}", e);
                            continue;
                        }
                    };
                    let r0 = match u32::from_str_radix(&plaintext[8..16], 16) {
                        Ok(value) => value,
                        Err(e) => {
                            println!("Error parsing hexadecimal: {:?}", e);
                            continue;
                        }
                    };
                    let l4 = match u32::from_str_radix(&ciphertext[0..8],16) {
                        Ok(value) => value,
                        Err(e) => {
                            println!("Error parsing hexadecimal: {:?}", e);
                            continue;
                        }
                    };
                    let r4 = match u32::from_str_radix(&ciphertext[8..16], 16) {
                        Ok(value) => value,
                        Err(e) => {
                            println!("Error parsing hexadecimal: {:?}", e);
                            continue;
                        }
                    };

                    let s_23_29: u32 = (((l0 ^ r0 ^ l4) >> 8) & 1) ^ (((l0 ^ r0 ^ l4) >> 2
                ) & 1);
                    let s_31: u32 = (l0 ^ l4 ^ r4) & 1;
                    let f_result: u32 = f(l0 ^ r0 ^ key) & 1;
                    let s_31_f_round = f_result;
                    let a = (s_23_29 as u32 ) ^ (s_31 as u32) ^ s_31_f_round;
                    //println!(" l0:{} r0:{} s_23_29:{} s_31:{} s_31_f_round:{}", l0,r0,s_23_29,s_31,f_result);
                    if a == 0 {
                        sum_zeros += 1;
                    }    
                    else if a == 1 {
                        sum_ones += 1;
                    }
                   
                    if sum_zeros > 5 && sum_ones > 5 {
                        break;
                    }
                    
                } else {
                    println!("Error: Invalid data format");
                }
            }
            if sum_zeros > 199 || sum_ones > 199 {
                println!("Found key: {}", key);
                println!("Zeros:{} Ones:{}", sum_zeros, sum_ones);
                found_keys.push(key);
            }
        }
    } else {
        println!("Unable to read file");
    }

    println!("Found keys: {:?}", found_keys);
}
