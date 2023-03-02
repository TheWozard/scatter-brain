#[derive(Debug, PartialEq, Eq)]
struct TokenCollection {
    tokens: Vec<String>,
    parent: Option<Box<TokenCollection>>,
}

impl Default for TokenCollection {
    fn default() -> Self {
        return TokenCollection { tokens: Vec::new(), parent: None }
    }
}

impl TokenCollection {
    fn child(self: Self) -> TokenCollection {
        return TokenCollection { tokens: Vec::new(), parent: Some(Box::new(self)) }
    }

    fn update(self: Self, tokens: Vec<String>) {

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_default_child() {
        let parent = TokenCollection::default();
        assert_eq!(parent.child().parent.unwrap(),  parent)
    }
}

