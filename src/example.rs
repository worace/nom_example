fn is_non_brace(a: char) -> bool {
    a != '{' && a != '}'
}

named!(content<&str, &str>, take_while!(is_non_brace));

named!(balanced_braces<&str, &str>,
       recognize!(delimited!(tag!("{"),
                             alt!(balanced_braces|content),
                             tag!("}")))
);

#[cfg(test)]
mod tests {
    use crate::example::balanced_braces;
    #[test]
    fn test_brace_matching() {
        let nested = "{a{b}a}";
        assert_eq!(Ok(("", "{inner}")), balanced_braces("{inner}"));
        assert_eq!(Ok(("", nested)), balanced_braces(nested));
    }
}
