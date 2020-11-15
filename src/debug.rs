use std::io::Write;

/// Some functions and methods take an instance of this enum as parameter to determine how much
/// effort (and thus time) it should spend on checking if the function/method call is correct and
/// makes sense.
///
/// Debug checks are sometimes very helpful, but some are quite expensive. This enum makes it easy
/// to enable debugging where you need it, and disable it where you don't (you only have to change
/// a single line of code to toggle it).
///
/// *Minimal* is the lowest debug level and will yield the best performance. *All* is the highest
/// debug level and will yield the worst performance. The cost and gains of debugging will differ
/// from method to method because every method has different debugging checks.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum DebugLevel {
    /// Only the absolutely necessary checks should be done. Use this if you are certain you are
    /// using the function or method correctly.
    Minimal,
    /// Only very cheap checks and absolutely necessary checks should be done. This should rarely
    /// lead to any performance issues, but might catch some errors for you.
    Low,
    /// Only do debug checks that are not much more expensive than the operation itself. This can
    /// cause a noticeable performance cost compared to *Low* and *Minimal*, but can catch many
    /// more errors.
    Basic,
    /// Do most of the debug checks, even those that can be significantly more expensive than the
    /// operation itself.
    High,
    /// Do all debug checks.
    All,
}

pub(crate) fn log(writer: &mut Option<&mut dyn Write>, source: &str, message: &str) {
    if writer.is_some() {
        writeln!(
            writer.as_mut().unwrap(),
            "[Griphin] [{}]: {}",
            source,
            message
        )
        .unwrap();
    } else {
        println!("[Griphin] [{}]: {}", source, message);
    }
}
