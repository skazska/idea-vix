/// Build a comma-separated SQL SET clause by evaluating boolean conditions.
///
/// Example:
/// let set = sqlx_build_set!(
///     item.name.is_some() => "name = ?",
///     item.description.is_some() => "description = ?",
///     item.icon.is_some() => "icon = ?",
///     item.is_public.is_some() => "is_public = ?"
/// );
/// if set.is_empty() { /* error: no fields */ }
/// TODO extend to handle binding values
#[macro_export]
macro_rules! sqlx_build_set {
    ( $( $cond:expr => $frag:expr ),+ $(,)? ) => {{
        let mut parts = ::std::vec::Vec::new();
        $( if $cond { parts.push($frag); } )+
        parts.join(", ")
    }}
}

#[cfg(test)]
mod tests {
    #[test]
    fn build_set_none() {
        let set = crate::sqlx_build_set!( false => "name = ?", false => "icon = ?" );
        assert!(set.is_empty());
    }

    #[test]
    fn build_set_some() {
        let set = crate::sqlx_build_set!( true => "name = ?", false => "icon = ?", true => "is_public = ?" );
        assert_eq!(set, "name = ?, is_public = ?");
    }
}
