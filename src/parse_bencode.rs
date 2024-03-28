pub fn decode_bencoded_value(encoded_value: &str) -> (serde_json::Value, &str) {
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
                // TODO: read more on the .into() bcoz i dont understand it
                // https://dev.to/richardbray/type-conversion-in-rust-as-from-and-into-493l
                // https://www.reddit.com/r/rust/comments/p69c9w/what_does_the_into_method_do/
                // https://doc.rust-lang.org/rust-by-example/conversion/from_into.html
                return  (n.into(), rest);
            }
        }

        Some('d') => {
            let mut dict = serde_json::Map::new(); // create an empty vec/list
            let mut rest = encoded_value.split_at(1).1; // get rest of list from the letter 'l',

            // first check that rest &str is not empty and it doesnt start with e
            while !rest.is_empty() && !rest.starts_with('e') {
                let (k, remainder) = decode_bencoded_value(rest);

                // use shadowing to replace k with something else 
                // check to see if the k is of type enum variant string and return the string 
                // if not panic the whole program and exit
                let k = match k { 
                    serde_json::Value::String(k) => k,
                        _ => panic!("Dict keys must be of type string")
                };

                let (v, remainder) = decode_bencoded_value(remainder);
                dict.insert(k, v); // saves decoded values of the list into a vector
                rest = remainder;
            }

            return (serde_json::Value::Object(dict.into()), &rest[1..])
        }

        Some('l') => {
            // Example  of list li25e3:fooe
            let mut values = Vec::new(); // create an empty vec/list
            let mut rest = encoded_value.split_at(1).1; // get rest of list from the letter 'l',
                                                        // in this case it will  i25e3:fooe
                                                        // println!("REST of List: {}",rest.starts_with('e'));

                                                        // first check that rest &str is not empty and it doesnt start with e
            while !rest.is_empty() && !rest.starts_with('e') {
                let (v, remainder) = decode_bencoded_value(rest);
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


