use std::{error::Error, fs};

fn parse_message(
    end: usize,
    packet: &[bool],
    outmost: bool,
    version_sum: &mut i32,
) -> (usize, Vec<u64>) {
    let mut pos: usize = 0;
    let mut res_out = Vec::new();
    while pos + 7 < end {
        let mut version: u8 = 0;
        for _ in 0..3 {
            version = version << 1;
            if packet[pos] {
                version |= 1;
            }
            pos += 1;
        }
        let mut type_id: u8 = 0;
        for _ in 0..3 {
            type_id = type_id << 1;
            if packet[pos] {
                type_id |= 1;
            }
            pos += 1;
        }

        println!("version: {}, type_id: {}", version, type_id);
        *version_sum += version as i32;

        match type_id {
            4 => {
                // Literal value
                let mut value: u64 = 0;
                let mut last_one = true;
                while last_one && (packet[pos] || last_one) {
                    if !packet[pos] {
                        last_one = false;
                    }
                    pos += 1;
                    for _ in 0..4 {
                        value = value << 1;
                        if packet[pos] {
                            value |= 1;
                        }
                        pos += 1;
                    }
                }
                println!("value: {}", value);
                res_out.push(value);
                if outmost {
                    while pos % 4 != 0 {
                        println!("chomp!!");
                        pos += 1;
                    }
                }
            }
            _ => {
                // Operator

                println!("operator start! {}", type_id);

                let sub_type_id = packet[pos];
                pos += 1;
                let mut sub_values = Vec::new();
                if !sub_type_id {
                    let mut length: usize = 0;
                    for _ in 0..15 {
                        length = length << 1;
                        if packet[pos] {
                            length |= 1;
                        }
                        pos += 1;
                    }
                    println!("length: {}", length);
                    let res = parse_message(length, &packet[pos..], false, version_sum);
                    pos += res.0;
                    sub_values.extend(res.1);
                } else {
                    println!("num type");
                    let mut sub_packets: usize = 0;
                    for _ in 0..11 {
                        sub_packets = sub_packets << 1;
                        if packet[pos] {
                            sub_packets |= 1;
                        }
                        pos += 1;
                    }
                    for _ in 0..sub_packets {
                        let res = parse_message(8, &packet[pos..], false, version_sum);
                        pos += res.0;
                        sub_values.extend(res.1);
                    }
                }

                println!("sub_values: {:?}", sub_values);

                match type_id {
                    0 => {
                        let mut new_sub_val = 0;
                        for sub_val in sub_values {
                            new_sub_val += sub_val;
                        }
                        println!("sum!: {}", new_sub_val);
                        res_out.push(new_sub_val);
                    }
                    1 => {
                        let mut new_sub_val;
                        new_sub_val = sub_values[0];
                        for sub_val in &sub_values[1..] {
                            new_sub_val *= sub_val;
                        }
                        println!("product!: {}", new_sub_val);
                        res_out.push(new_sub_val);
                    }
                    2 => {
                        let mut new_sub_val = u64::MAX;
                        for sub_val in sub_values {
                            if sub_val < new_sub_val {
                                new_sub_val = sub_val;
                            }
                        }
                        println!("min!: {}", new_sub_val);
                        res_out.push(new_sub_val);
                    }
                    3 => {
                        let mut new_sub_val = 0;
                        for sub_val in sub_values {
                            if sub_val > new_sub_val {
                                new_sub_val = sub_val;
                            }
                        }
                        println!("max!: {}", new_sub_val);
                        res_out.push(new_sub_val);
                    }
                    5 => {
                        let new_sub_val = if sub_values[0] > sub_values[1] { 1 } else { 0 };
                        println!("greater than!: {}", new_sub_val);
                        res_out.push(new_sub_val);
                    }
                    6 => {
                        let new_sub_val = if sub_values[0] < sub_values[1] { 1 } else { 0 };
                        println!("less than!: {}", new_sub_val);
                        res_out.push(new_sub_val);
                    }
                    7 => {
                        let new_sub_val = if sub_values[0] == sub_values[1] { 1 } else { 0 };
                        println!("less than!: {}", new_sub_val);
                        res_out.push(new_sub_val);
                    }
                    _ => {
                        panic!("wrong type_id");
                    }
                }
            }
        }
    }

    (pos, res_out)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input: Vec<u8> = fs::read_to_string("input")?
        .chars()
        .map(|c| c.to_string())
        .map(|s| u8::from_str_radix(&s, 16).unwrap())
        .collect();

    let mut packet = Vec::new();
    for mut d in input {
        for _ in 0..4 {
            packet.push(d & 8 == 8);
            d = d << 1;
        }
    }

    let mut version_sum = 0;

    let res = parse_message(packet.len(), &packet[..], true, &mut version_sum);

    println!("{:?}", res.1);

    Ok(())
}
