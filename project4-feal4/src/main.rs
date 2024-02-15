use std::fs;

fn f(x0: u32, x1: u32, x2: u32, x3: u32) -> (u32, u32, u32, u32) {
    fn g0(a: u32, b: u32) -> u32 {
        let result: u32 = (a.wrapping_add(b)) % 256;
        (result << 2) | (result >> 6)
    }

    fn g1(a: u32, b: u32) -> u32 {
        let result = (a.wrapping_add(b).wrapping_add(1)) % 256;
        (result << 2) | (result >> 6)
    }

    let y0 = g0(x0, x1);
    let y1 = g1(x0 ^ x1, x2 ^ x3);
    let y2 = g0(y1, x2 ^ x3);
    let y3 = g1(y2, x3);

    (y0, y1, y2, y3)
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

        for key in 1..=u32::pow(2, 31) {
            let mut sum_zeros = 0;
            let mut sum_ones = 0;
            for text in data.chunks(2) {
                if let [plaintext, ciphertext] = text {
                    let l0 = match u32::from_str_radix(&plaintext[0..7], 16) {
                        Ok(value) => value,
                        Err(e) => {
                            println!("Error parsing hexadecimal: {:?}", e);
                            continue;
                        }
                    };
                    let r0 = match u32::from_str_radix(&plaintext[8..15], 16) {
                        Ok(value) => value,
                        Err(e) => {
                            println!("Error parsing hexadecimal: {:?}", e);
                            continue;
                        }
                    };
                    let l4 = match u32::from_str_radix(&ciphertext[0..7], 16) {
                        Ok(value) => value,
                        Err(e) => {
                            println!("Error parsing hexadecimal: {:?}", e);
                            continue;
                        }
                    };
                    let r4 = match u32::from_str_radix(&ciphertext[8..15], 16) {
                        Ok(value) => value,
                        Err(e) => {
                            println!("Error parsing hexadecimal: {:?}", e);
                            continue;
                        }
                    };

                    let s_23_29: u32 = (l0 ^ r0 ^ l4);
                    let s_31: u32 = (l0 ^ l4 ^ r4);
                    let f_result: (u32, u32, u32, u32) = f(l0 ^ r0 ^ key, 0, 0, 0);
                    let s_31_f_round = (f_result.0);
                    let a = (s_23_29 ^ s_31 ^ s_31_f_round);
                    if a == 0 {
                        sum_zeros += 1;
                    }    
                    else {
                        sum_ones += 1;
                    }

                    if sum_zeros > 20 && sum_ones > 20 {
                        break;
                        }
                    
                } else {
                    println!("Error: Invalid data format");
                }
            }
            if sum_zeros > 195 || sum_ones > 195 {
                println!("Found key: {}", key);
                found_keys.push(key);
            }
        }
    } else {
        println!("Unable to read file");
    }

    println!("Found keys: {:?}", found_keys);
}
