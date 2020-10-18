use std::ops::Deref;

/// A simple enum that either wraps an *&'static str* or a *String*. This type is
/// convenient as string type because both wrapped types can be occasionally
/// useful. To get an instance of this enum, use *enum_str* or *enum_string*.
/// 
/// # Example
/// ```
/// use griphin::*;
/// 
/// let static_ref = enum_str("Some string");
/// let owned = enum_string(String::from("Some string"));
/// assert_eq!(static_ref, owned);
/// ```
#[derive(Debug, Clone, Eq)]
pub enum EnumString {

    StaticRef(&'static str),
    Owned(String)
}

impl Deref for EnumString {

    type Target = str;

    fn deref(&self) -> &str {
        match self {
            Self::StaticRef(static_ref) => static_ref,
            Self::Owned(owned) => &owned
        }
    }
}

impl PartialEq for EnumString {

    fn eq(&self, other: &EnumString) -> bool {
        self.deref() == other.deref()
    }
}

impl PartialEq<&str> for EnumString {

    fn eq(&self, other: &&str) -> bool {
        self.deref() == *other
    }
}

/// Creates an *EnumString* instance that wraps the given static *str* reference.
pub const fn enum_str(string: &'static str) -> EnumString {
    EnumString::StaticRef(string)
}

/// Creates an *EnumString* instance that wraps the given *String*.
pub fn enum_string(string: String) -> EnumString {
    EnumString::Owned(string)
}

#[cfg(test)]
mod tests {

    use super::*;

    const HELLO_WORLD: EnumString = enum_str("HelloWorld");

    #[test]
    fn test_static_ref() {
        assert_eq!("HelloWorld", HELLO_WORLD.deref());
        assert_eq!(HELLO_WORLD, "HelloWorld");
        assert_eq!(HELLO_WORLD, enum_str("HelloWorld"));
        assert_eq!(HELLO_WORLD, enum_string(String::from("HelloWorld")));
    }

    #[test]
    fn test_string() {
        let hello_world = enum_string(String::from("HelloWorld"));
        assert_eq!("HelloWorld", hello_world.deref());
        assert_eq!(hello_world, "HelloWorld");
        assert_eq!(hello_world, enum_string(String::from("HelloWorld")));
        assert_eq!(hello_world, enum_str("HelloWorld"));
    }
}