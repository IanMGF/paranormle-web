use std::cmp::Ordering;

pub trait Comparable<R> {
    fn compare(&self, other: &Self) -> R;
    fn css(a: &R) -> &'static str;
}

impl<T> Comparable<bool> for T
where
    T: Eq + PartialEq,
{
    fn compare(&self, other: &T) -> bool {
        self == other
    }

    fn css(eq: &bool) -> &'static str {
        if *eq {
            "correct"
        } else {
            "incorrect"
        }
    }
}

impl<T> Comparable<Ordering> for T
where
    T: Ord,
{
    fn compare(&self, other: &T) -> Ordering {
        self.cmp(other)
    }

    fn css(cmp: &Ordering) -> &'static str {
        match cmp {
            Ordering::Equal => "correct",
            Ordering::Less => "less",
            Ordering::Greater => "greater",
        }
    }
}

pub struct InfoPiece<T: Comparable<C> + Clone, C>(T, C);
impl<T: Comparable<C> + Clone, C> InfoPiece<T, C> {
    pub fn compare(a: &T, b: &T) -> C {
        T::compare(a, b)
    }

    pub fn get_css(&self) -> &'static str {
        T::css(&self.1)
    }

    pub fn from_attempt(guess: &T, result: C) -> Self {
        InfoPiece(guess.clone(), result)
    }

    pub fn from_comparison(guess: &T, correct: &T) -> Self {
        let result = Self::compare(guess, correct);
        Self::from_attempt(guess, result)
    }
}
