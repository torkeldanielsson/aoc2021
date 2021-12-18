use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input: Vec<String> = fs::read_to_string("input")?
        .split(", y=")
        .map(|s| s.to_owned())
        .collect();
    let target_x: Vec<i32> = input[0]
        .strip_prefix("target area: x=")
        .unwrap()
        .split("..")
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    let target_y: Vec<i32> = input[1]
        .split("..")
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let mut hits = 0;

    for x in -1000..1000 {
        'y: for y in -1000..1000 {
            let mut x_vel = x;
            let mut y_vel = y;
            let mut pos = (0, 0);
            let mut maybe_max_y = 0;
            for _ in 0..1000 {
                pos.0 += x_vel;
                pos.1 += y_vel;
                if x_vel > 0 {
                    x_vel -= 1;
                }
                if x_vel < 0 {
                    x_vel += 1;
                }
                y_vel -= 1;

                if pos.1 > maybe_max_y {
                    maybe_max_y = pos.1;
                }

                if pos.0 >= target_x[0]
                    && pos.0 <= target_x[1]
                    && pos.1 >= target_y[0]
                    && pos.1 <= target_y[1]
                {
                    hits += 1;
                    println!("{}, {}", x, y);
                    continue 'y;
                }
            }
        }
    }

    println!("{}", hits);

    Ok(())
}
