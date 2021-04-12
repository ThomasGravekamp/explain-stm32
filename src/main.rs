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
    let series: String = name.chars().take(2).collect();

    if series.eq("F0") {
        return Option::Some(Series::F0)
    }
    else if series.eq("F1") {
        return Option::Some(Series::F1)
    }
    else if series.eq("F2") {
        return Option::Some(Series::F2)
    }
    else if series.eq("F3") {
        return Option::Some(Series::F3)
    }
    else if series.eq("F4") {
        return Option::Some(Series::F4)
    }
    else if series.eq("F7") {
        return Option::Some(Series::F7)
    }

    Option::None
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

fn main() {
    println!("Enter a valid STM32 device name:");

    // name will be consumed by the functions it is passed to
    let mut name = String::new();

    // Read user input
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line.");

    // Find device family
    let family = get_device_family(&mut name);
    display_device_family(&family);

    if family.is_none() {
        return;
    }

    let series = get_device_series(&mut name);
    if let Some(series) = series {
        display_device_series(series);
    }
    else {
        return;
    }

    println!("Debug: {}", name);
}
