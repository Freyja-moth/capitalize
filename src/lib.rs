/// ```rust
/// # use capitalize::Capitalize;
/// let string = "heLLo WoRld!";
///
/// let new_string: String = string.capitalize().collect();
///
/// assert_eq!(new_string, "Hello world!");
/// assert!(new_string.is_capitalized());
/// ```
pub trait Capitalize
where
    Self: Sized + AsRef<str>,
{
    fn capitalize(&self) -> impl Iterator<Item = char>;
    fn is_capitalized(&self) -> bool {
        itertools::equal(self.as_ref().chars(), self.capitalize())
    }
}

impl<T> Capitalize for T
where
    T: AsRef<str>,
{
    fn capitalize(&self) -> impl Iterator<Item = char> {
        let mut chars = self.as_ref().chars();

        chars
            .next()
            .unwrap_or_default()
            .to_uppercase()
            .chain(chars.map(|c| c.to_ascii_lowercase()))
    }
}
