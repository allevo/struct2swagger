use crate::JsonSchemaDefinition;

macro_rules! impl_json_schema_definition {
    ($rt:ty, "integer", $min:expr, $max:expr) => {
        impl JsonSchemaDefinition for $rt {
            fn get_schema_type() -> serde_json::Value {
                json!({
                    "type": "integer",
                    "minimum": $min,
                    "maximum": $max,
                })
            }
        }
    };
    ($rt:ty, "number", $min:expr, $max:expr) => {
        impl JsonSchemaDefinition for $rt {
            fn get_schema_type() -> serde_json::Value {
                json!({
                    "type": "number",
                    "minimum": $min,
                    "maximum": $max,
                })
            }
        }
    };
    ($rt:ty, "boolean") => {
        impl JsonSchemaDefinition for $rt {
            fn get_schema_type() -> serde_json::Value {
                json!({
                    "type": "boolean"
                })
            }
        }
    };
    ($rt:ty, "string") => {
        impl JsonSchemaDefinition for $rt {
            fn get_schema_type() -> serde_json::Value {
                json!({
                    "type": "string",
                })
            }
        }
    };
    ($rt:ty, "array", "integer", $min:expr, $max:expr) => {
        impl JsonSchemaDefinition for $rt {
            fn get_schema_type() -> serde_json::Value {
                json!({
                    "type": "array",
                    "items": {
                        "type": "integer",
                        "minimum": $min,
                        "maximum": $max,
                    },
                })
            }
        }
    };
}
impl_json_schema_definition!(i8, "integer", std::i8::MIN, std::i8::MAX);
impl_json_schema_definition!(i16, "integer", std::i16::MIN, std::i16::MAX);
impl_json_schema_definition!(i32, "integer", std::i32::MIN, std::i32::MAX);
impl_json_schema_definition!(i64, "integer", std::i64::MIN, std::i64::MAX);
impl_json_schema_definition!(i128, "integer", std::i128::MIN, std::i128::MAX);
impl_json_schema_definition!(u8, "integer", std::u8::MIN, std::u8::MAX);
impl_json_schema_definition!(u16, "integer", std::u16::MIN, std::u16::MAX);
impl_json_schema_definition!(u32, "integer", std::u32::MIN, std::u32::MAX);
impl_json_schema_definition!(u64, "integer", std::u64::MIN, std::u64::MAX);
impl_json_schema_definition!(u128, "integer", std::u128::MIN, std::u128::MAX);
impl_json_schema_definition!(isize, "integer", std::isize::MIN, std::isize::MAX);
impl_json_schema_definition!(usize, "integer", std::usize::MIN, std::usize::MAX);

impl_json_schema_definition!(f32, "number", std::f32::MIN, std::f32::MAX);
impl_json_schema_definition!(f64, "number", std::f64::MIN, std::f64::MAX);

impl_json_schema_definition!(bool, "boolean");

impl_json_schema_definition!(String, "string");
impl_json_schema_definition!(&str, "string");

impl_json_schema_definition!([i8], "array", "integer", std::i8::MIN, std::i8::MAX);
impl_json_schema_definition!([i16], "array", "integer", std::i16::MIN, std::i16::MAX);
impl_json_schema_definition!([i32], "array", "integer", std::i32::MIN, std::i32::MAX);
impl_json_schema_definition!([i64], "array", "integer", std::i64::MIN, std::i64::MAX);
impl_json_schema_definition!([i128], "array", "integer", std::i128::MIN, std::i128::MAX);
impl_json_schema_definition!([u8], "array", "integer", std::u8::MIN, std::u8::MAX);
impl_json_schema_definition!([u16], "array", "integer", std::u16::MIN, std::u16::MAX);
impl_json_schema_definition!([u32], "array", "integer", std::u32::MIN, std::u32::MAX);
impl_json_schema_definition!([u64], "array", "integer", std::u64::MIN, std::u64::MAX);
impl_json_schema_definition!([u128], "array", "integer", std::u128::MIN, std::u128::MAX);
impl_json_schema_definition!(
    [isize],
    "array",
    "integer",
    std::isize::MIN,
    std::isize::MAX
);
impl_json_schema_definition!(
    [usize],
    "array",
    "integer",
    std::usize::MIN,
    std::usize::MAX
);

impl<T: JsonSchemaDefinition> JsonSchemaDefinition for Option<T> {
    fn get_schema_type() -> serde_json::Value {
        <T>::get_schema_type()
    }
}

impl<T: JsonSchemaDefinition> JsonSchemaDefinition for Vec<T> {
    fn get_schema_type() -> serde_json::Value {
        json!({
            "type": "array",
            "items": T::get_schema_type(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Field;

    impl PartialEq for Field {
        fn eq(&self, other: &Self) -> bool {
            self.name == other.name
        }
    }

    macro_rules! test {
        ($rt:ty, $expected: expr) => {
            let actual = <$rt>::get_schema_type();
            assert_eq!(actual, $expected);
        };
    }

    #[test]
    fn get_schema_type_u8() {
        test!(
            u8,
            json!({
                "type": "integer",
                "minimum": 0,
                "maximum": 255,
            })
        );
    }

    #[test]
    fn get_schema_type_i8() {
        test!(
            i8,
            json!({
                "type": "integer",
                "minimum": -128,
                "maximum": 127,
            })
        );
    }

    #[test]
    fn get_schema_type_f32() {
        test!(
            f32,
            json!({
                "type": "number",
                "minimum": -340_282_346_638_528_860_000_000_000_000_000_000_000.0,
                "maximum": 340_282_346_638_528_860_000_000_000_000_000_000_000.0,
            })
        );
    }

    #[test]
    fn get_schema_type_f64() {
        test!(
            f64,
            json!({
                "type": "number",
                "minimum": -179_769_313_486_231_570_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000.0,
                "maximum": 179_769_313_486_231_570_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000.0,
            })
        );
    }

    #[test]
    fn get_schema_type_boolean() {
        test!(
            bool,
            json!({
                "type": "boolean",
            })
        );
    }

    #[test]
    fn get_schema_type_string() {
        test!(
            String,
            json!({
                "type": "string",
            })
        );
    }

    #[test]
    fn get_schema_type_str() {
        test!(
            &str,
            json!({
                "type": "string",
            })
        );
    }

    #[test]
    fn get_schema_type_option() {
        test!(
            Option<String>,
            json!({
                "type": "string",
            })
        );
    }
}
