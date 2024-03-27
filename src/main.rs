fn decode_bencoded_value(encoded_value: &str) -> (serde_json::Value, &str) {
    match encoded_value.chars().next() {
        Some('i') => {
            if let Some((n, rest)) =  encoded_value
                .split_at(1).1
                .split_once('e')
                .and_then(|(digits, rest)| {
                    let n = digits.parse::<i64>().ok()?;
                    Some((n, rest))
                })
            {
                return  (n.into(), rest);
            }
        }

        Some('l') => {
            // Example  of list li25e3:fooe
            let mut values = Vec::new(); // create an empty vec/list
            let mut rest = encoded_value.split_at(1).1; // get rest of list from the letter 'l',
            // in this case it will  i25e3:fooe
            // println!("REST of List: {}",rest.starts_with('e'));

            // first check that rest &str is not empty and it doesnt start with e
            while !rest.is_empty() && !rest.starts_with('e') {
                // println!("REST of List: {rest}");
                let (v, remainder) = decode_bencoded_value(rest);
                println!("v: {v} remainder: {remainder}");
                values.push(v); // saves decoded values of the list into a vector
                rest = remainder;
            }
            
            return (values.into(), &rest[1..])
        }

        Some('0'..='9') => {
            if let Some((len, rest)) = encoded_value.split_once(':') { // get the length of string
                // and the string separately
                if let Ok(len) = len.parse::<usize>() { // convert the length of the string to u16
                    return (rest[..len].to_string().into(), &rest[len..])
                } 
            }
        }

        _ => {}
    }

    panic!("Unhandled encoded value: {}", encoded_value)
}



fn main() {
    let args: Vec<String> = std::env::args().collect();
    let command = &args[1];

    if command == "decode" {
        let encoded_value = &args[2];

        let decoded_value = decode_bencoded_value(encoded_value);
        println!("{}", decoded_value.0);

    } else {
        println!("unkown command: {:?}", command);
    }
}
