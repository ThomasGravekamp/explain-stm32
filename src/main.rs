use std::io;

mod sections;

enum SubFamily {
    F030,
    F070,
}

fn get_device_sub_family(name: &str) -> Option<(SubFamily, &str)> {
    let remainder = &name[3..];

    match &name[0..3] {
        "030" => Option::Some((SubFamily::F030, remainder)),
        "070" => Option::Some((SubFamily::F070, remainder)),
        _ => Option::None
    }
}

fn display_device_sub_family(sub_family: SubFamily) {
    match sub_family {
        SubFamily::F030 => println!("Sub-family: 030"),
        SubFamily::F070 => println!("Sub-family: 070"),
    }
}

fn main() {
    println!("Enter a valid STM32 device name:");

    // name will be consumed by the functions it is passed to
    let mut name = String::new();

    // Read user input
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line.");

    let mut remainder: &str;

    // Find and display device family
    let family = sections::family::get_device_family(&name);
    if let Some((family, re)) = family {
        sections::family::display_device_family(family);
        remainder = re;
    }
    else {
        return;
    }

    // Find and display device series
    let series = sections::series::get_device_series(remainder);
    if let Some((series, re)) = series {
        sections::series::display_device_series(series);
        remainder = re;
    }
    else {
        return;
    }

    let sub_family = get_device_sub_family(remainder);
    if let Some((sub_family, re)) = sub_family {
        display_device_sub_family(sub_family);
        remainder = re;
    }
    else {
        return;
    }

    println!("Debug: {}", name);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut test_name = String::from("F103C8T6");

        assert_eq!(sections::series::get_device_series(&mut test_name).unwrap(), sections::series::Series::F1);
    }
}
