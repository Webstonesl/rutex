pub struct LanguageString<'a> {
    language: &'a str,
    locale: Option<&'a str>,
    dialect: Option<&'a str>,
}
impl<'a> TryFrom<&'a str> for LanguageString<'a> {
    type Error = ();

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let c = value.chars().filter(|a| matches!(a, '_' | '-')).count()
            + if !value.is_empty() { 1 } else { 0 };

        if c == 0 {
            todo!("Create error type")
        }
        let mut language: Option<&str> = None;
        let mut locale = None;
        let mut dialect = None;
        for (i, l) in value.split(['_', '-']).enumerate() {
            if i == 0 {
                language = Some(l);
            } else if l.chars().all(|a| a.is_uppercase() || !a.is_alphabetic()) {
                locale = Some(l);
            } else {
                dialect = Some(l);
            }
        }
        let language = match language {
            Some(a) => a,
            None => todo!(),
        };
        Ok(Self {
            language,
            locale,
            dialect,
        })
    }
}
