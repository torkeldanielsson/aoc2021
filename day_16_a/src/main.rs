use std::{error::Error, fs};

fn parse_message(end: usize, packet: &[bool], outmost: bool, version_sum: &mut i32) -> usize {
    let mut pos: usize = 0;
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
                if outmost {
                    while pos % 4 != 0 {
                        println!("chomp!!");
                        pos += 1;
                    }
                }
            }
            _ => {
                // Operator
                let type_id = packet[pos];
                pos += 1;
                if !type_id {
                    let mut length: usize = 0;
                    for _ in 0..15 {
                        length = length << 1;
                        if packet[pos] {
                            length |= 1;
                        }
                        pos += 1;
                    }
                    println!("length: {}", length);
                    pos += parse_message(length, &packet[pos..], false, version_sum);
                } else {
                    let mut sub_packets: usize = 0;
                    for _ in 0..11 {
                        sub_packets = sub_packets << 1;
                        if packet[pos] {
                            sub_packets |= 1;
                        }
                        pos += 1;
                    }
                    for _ in 0..sub_packets {
                        pos += parse_message(8, &packet[pos..], false, version_sum);
                    }
                }
            }
        }
    }

    pos
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

    parse_message(packet.len(), &packet[..], true, &mut version_sum);

    println!("{}", version_sum);

    Ok(())
}
