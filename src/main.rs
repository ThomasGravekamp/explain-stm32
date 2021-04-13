use std::io;

// Allowed values: "STM32"
fn get_device_family(name: &mut String) -> Option<String> {
    let expected_string = String::from("STM32");
    let expected_chars = expected_string.chars();
    let actual_chars = name.chars();

    let iterator = expected_chars.zip(actual_chars);

    for it in iterator {
        let (ia, ib) = it;

        if ia != ib {
            return Option::None;
        }
    }

    // Hard coded because the name is always expected to start with STM32
    name.replace_range(0..5, "");

    Option::Some(String::from("STM32"))
}

fn display_device_family(family: &Option<String>) {
    match family {
        Some(family) => println!("Family: {}", family),
        None => println!("Unknown device family.")
    }
}

#[derive(Debug, PartialEq)]
enum Series {
    F0,
    F1,
    F2,
    F3,
    F4,
    F7
}

// Allowed values: "F0", "F1", "F2", "F3", "F4", "F7"
fn get_device_series(name: &mut String) -> Option<Series> {
    // Get the first two characters from the name string
    let series: String = name.chars().take(2).collect();

    // Remove first char from name string
    name.replace_range(0..1, "");

    // Match on the first two characters and return the corresponding Series value
    match series.as_str() {
        "F0" => Option::Some(Series::F0),
        "F1" => Option::Some(Series::F1),
        "F2" => Option::Some(Series::F2),
        "F3" => Option::Some(Series::F3),
        "F4" => Option::Some(Series::F4),
        "F7" => Option::Some(Series::F7),
        _ => Option::None
    }
}

fn display_device_series(series: Series) {
    match series {
        Series::F0 => println!("Series: F0"),
        Series::F1 => println!("Series: F1"),
        Series::F2 => println!("Series: F2"),
        Series::F3 => println!("Series: F3"),
        Series::F4 => println!("Series: F4"),
        Series::F7 => println!("Series: F7")
    }
}

enum SubFamily {
    F030,
    F070,
}

fn get_device_sub_family(name: &mut String) -> Option<SubFamily> {
    let sub_family: String = name.chars().take(3).collect();

    name.replace_range(0..4, "");

    match sub_family.as_str() {
        "030" => Option::Some(SubFamily::F030),
        "070" => Option::Some(SubFamily::F070),
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

    // Find and display device family
    let family = get_device_family(&mut name);
    display_device_family(&family);

    if family.is_none() {
        return;
    }

    // Find and display device series
    let series = get_device_series(&mut name);
    if let Some(series) = series {
        display_device_series(series);
    }
    else {
        return;
    }

    let sub_family = get_device_sub_family(&mut name);
    if let Some(sub_family) = sub_family {
        display_device_sub_family(sub_family);
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

        assert_eq!(get_device_series(&mut test_name).unwrap(), Series::F1);
    }
}
