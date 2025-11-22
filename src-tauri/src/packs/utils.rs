use serde::Deserializer;

pub fn deserialize_f64_from_string<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Visitor;
    use std::fmt;

    struct F64Visitor;

    impl<'de> Visitor<'de> for F64Visitor {
        type Value = f64;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a number or a string representing a number")
        }

        fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(value)
        }

        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(value as f64)
        }

        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(value as f64)
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            value.parse::<f64>().map_err(|_| {
                E::invalid_value(serde::de::Unexpected::Str(value), &"a string representing a number")
            })
        }
    }

    deserializer.deserialize_any(F64Visitor)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortField {
    Name,
    Popularity,
    Overall,
    Stream,
    Jumpstream,
    Handstream,
    Jacks,
    Chordjacks,
    Stamina,
    Technical,
}

impl SortField {
    pub fn as_str(&self) -> &'static str {
        match self {
            SortField::Name => "name",
            SortField::Popularity => "popularity",
            SortField::Overall => "overall",
            SortField::Stream => "stream",
            SortField::Jumpstream => "jumpstream",
            SortField::Handstream => "handstream",
            SortField::Jacks => "jacks",
            SortField::Chordjacks => "chordjacks",
            SortField::Stamina => "stamina",
            SortField::Technical => "technical",
        }
    }
    
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "name" => Some(SortField::Name),
            "popularity" => Some(SortField::Popularity),
            "overall" => Some(SortField::Overall),
            "stream" => Some(SortField::Stream),
            "jumpstream" => Some(SortField::Jumpstream),
            "handstream" => Some(SortField::Handstream),
            "jacks" => Some(SortField::Jacks),
            "chordjacks" => Some(SortField::Chordjacks),
            "stamina" => Some(SortField::Stamina),
            "technical" => Some(SortField::Technical),
            _ => None,
        }
    }
}

pub fn build_sort_string(field: SortField, descending: bool) -> String {
    if descending {
        format!("-{}", field.as_str())
    } else {
        field.as_str().to_string()
    }
}

