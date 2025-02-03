#[allow(dead_code)]
#[derive(Debug)]
struct ColorRegularStruct {
    red: u8,
    green: u8,
    blue: u8,
}

#[allow(dead_code)]
struct ColorTupleStruct(u8, u8, u8);

#[allow(dead_code)]
#[derive(Debug)]
struct UnitStruct;

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regular_structs() {
        // Instantiate a regular struct with RGB values for green
        let green = ColorRegularStruct {
            red: 0,
            green: 255,
            blue: 0,
        };

        // Test assertions for the `green` struct
        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // Instantiate a tuple struct with RGB values for green
        let green = ColorTupleStruct(0, 255, 0);

        // Test assertions for the tuple struct
        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // Instantiate a unit struct
        let unit_struct = UnitStruct;

        // Test assertion for the unit struct
        let message = format!("{unit_struct:?}s are fun!");

        assert_eq!(message, "UnitStructs are fun!");
    }
}
