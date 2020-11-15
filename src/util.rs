use std::fmt::{Display, Formatter};
use std::ops::Add;
use std::sync::Arc;

/// A simple enum that either wraps an *&'static str* or a *Arc<String>*. This type is
/// convenient as string type because both wrapped types can be occasionally
/// useful. To get an instance of this enum, use *str_ref* or *string_ref*.
///
/// # Example
/// ```
/// use griphin::*;
///
/// let static_ref = str_ref("Some string");
/// let owned = string_ref(String::from("Some string"));
/// assert_eq!(static_ref, owned);
/// ```
#[derive(Debug, Eq)]
pub enum StringRef {
    Static(&'static str),
    NonStatic(Arc<String>),
}

impl StringRef {
    pub fn to_str(&self) -> &str {
        match self {
            Self::Static(static_ref) => static_ref,
            Self::NonStatic(owned) => &owned,
        }
    }
}

impl PartialEq for StringRef {
    fn eq(&self, other: &StringRef) -> bool {
        self.to_str() == other.to_str()
    }
}

impl PartialEq<&str> for StringRef {
    fn eq(&self, other: &&str) -> bool {
        self.to_str() == *other
    }
}

impl Clone for StringRef {
    fn clone(&self) -> StringRef {
        match self {
            Self::Static(static_ref) => str_ref(static_ref),
            Self::NonStatic(counter) => StringRef::NonStatic(Arc::clone(&counter)),
        }
    }
}

impl Add<&StringRef> for &StringRef {
    type Output = StringRef;

    fn add(self, other: &StringRef) -> StringRef {
        let string = String::from(self.to_str()) + other.to_str();
        string_ref(string)
    }
}

impl Display for StringRef {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

/// Creates an *StringRef* instance that wraps the given static *str* reference.
pub const fn str_ref(string: &'static str) -> StringRef {
    StringRef::Static(string)
}

/// Creates an *StringRef* instance that wraps the given *String*.
pub fn string_ref(string: String) -> StringRef {
    StringRef::NonStatic(Arc::new(string))
}

#[cfg(test)]
mod tests {

    use super::*;

    const HELLO_WORLD: StringRef = str_ref("HelloWorld");

    #[test]
    fn test_static_ref() {
        assert_eq!("HelloWorld", HELLO_WORLD.to_str());
        assert_eq!(HELLO_WORLD, "HelloWorld");
        assert_eq!(HELLO_WORLD, str_ref("HelloWorld"));
        assert_eq!(HELLO_WORLD, string_ref(String::from("HelloWorld")));
        assert_eq!(HELLO_WORLD, HELLO_WORLD.clone());
    }

    #[test]
    fn test_string() {
        let hello_world = string_ref(String::from("HelloWorld"));
        assert_eq!("HelloWorld", hello_world.to_str());
        assert_eq!(hello_world, "HelloWorld");
        assert_eq!(hello_world, string_ref(String::from("HelloWorld")));
        assert_eq!(hello_world, str_ref("HelloWorld"));
        assert_eq!(hello_world, hello_world.clone());
    }
}
