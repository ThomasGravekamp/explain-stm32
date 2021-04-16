#[derive(Debug, PartialEq)]
pub enum Series {
    F0,
    F1,
    F2,
    F3,
    F4,
    F7,
}

// Allowed values: "F0", "F1", "F2", "F3", "F4", "F7"
pub fn get_device_series(name: &str) -> Option<(Series, &str)> {
    let remainder = &name[1..];

    // Match on the first two characters and return the corresponding Series value
    match &name[0..2] {
        "F0" => Option::Some((Series::F0, remainder)),
        "F1" => Option::Some((Series::F1, remainder)),
        "F2" => Option::Some((Series::F2, remainder)),
        "F3" => Option::Some((Series::F3, remainder)),
        "F4" => Option::Some((Series::F4, remainder)),
        "F7" => Option::Some((Series::F7, remainder)),
        _ => Option::None
    }
}

pub fn display_device_series(series: Series) {
    match series {
        Series::F0 => println!("Series: F0"),
        Series::F1 => println!("Series: F1"),
        Series::F2 => println!("Series: F2"),
        Series::F3 => println!("Series: F3"),
        Series::F4 => println!("Series: F4"),
        Series::F7 => println!("Series: F7")
    }
}