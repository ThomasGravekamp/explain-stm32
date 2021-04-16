// Allowed values: "STM32"

pub enum Family {
    STM32,
}

pub fn get_device_family(name: &String) -> Option<(Family, &str)> {
    if name.starts_with("STM32") {
        return Option::Some((Family::STM32, &name[5..]));
    }

    Option::None
}

pub fn display_device_family(family: Family) {
    match family {
        Family::STM32 => println!("Family: STM32")
    }
}