pub trait ToDigit {
    fn to_digit(&self) -> Option<u8>;
}

impl ToDigit for u8 {
    fn to_digit(&self) -> Option<u8> {
        if self.is_ascii_digit() {
            return Some(*self - b'0');
        }
        None
    }
}

pub trait Parsable<T>: Iterator {
    fn next_number(&mut self) -> Option<T>;
}

macro_rules! parsable_number {
    ($type:ident) => {
        impl<T: Iterator<Item = u8>> Parsable<$type> for T {
            fn next_number(&mut self) -> Option<$type> {
                let mut value: Option<$type> = None;
                for byte in self {
                    if let Some(digit) = byte.to_digit() {
                        if let Some(current) = value {
                            value = Some(current * 10 + digit as $type);
                        } else {
                            value = Some(digit as $type);
                        }
                    } else if value.is_some() {
                        return value;
                    }
                }

                value
            }
        }
    };
}

macro_rules! parsable_negative_number {
    ($type:ident) => {
        impl<T: Iterator<Item = u8>> Parsable<$type> for T {
            fn next_number(&mut self) -> Option<$type> {
                let mut negative = false;
                let mut value: Option<$type> = None;
                for byte in self {
                    if let Some(digit) = byte.to_digit() {
                        if let Some(current) = value {
                            value = Some(current * 10 + digit as $type);
                        } else {
                            value = Some(digit as $type);
                        }
                    } else if let Some(value) = value {
                        if negative {
                            return Some(-value);
                        }
                        return Some(value);
                    } else if byte == b'-' {
                        negative = true;
                    } else {
                        negative = false;
                    }
                }

                if let Some(value) = value {
                    if negative {
                        return Some(-value);
                    }
                    return Some(value);
                }
                None
            }
        }
    };
}

macro_rules! parsable_float_number {
    ($type:ident) => {
        impl<T: Iterator<Item = u8>> Parsable<$type> for T {
            fn next_number(&mut self) -> Option<$type> {
                let mut negative = false;
                let mut value: Option<$type> = None;
                for byte in self {
                    if let Some(digit) = byte.to_digit() {
                        if let Some(current) = value {
                            value = Some(current * 10.0 + digit as $type);
                        } else {
                            value = Some(digit as $type);
                        }
                    } else if let Some(value) = value {
                        if negative {
                            return Some(-value);
                        }
                        return Some(value);
                    } else if byte == b'-' {
                        negative = true;
                    } else {
                        negative = false;
                    }
                }

                if let Some(value) = value {
                    if negative {
                        return Some(-value);
                    }
                    return Some(value);
                }
                None
            }
        }
    };
}

parsable_number!(u16);
parsable_number!(u32);
parsable_number!(u64);
parsable_number!(u128);
parsable_number!(usize);
parsable_float_number!(f32);
parsable_float_number!(f64);
parsable_negative_number!(i16);
parsable_negative_number!(i32);
parsable_negative_number!(i64);
parsable_negative_number!(i128);
parsable_negative_number!(isize);
