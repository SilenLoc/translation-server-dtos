use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct TransReq {
    pub content: String,
    pub from: String,
    pub to: String,
}

impl TransReq {
    pub fn new(content: &str, from: &str, to: &str) -> TransReq {
        TransReq {
            content: content.to_string(),
            from: from.to_string(),
            to: to.to_string(),
        }
    }

    pub fn example() -> String {
        serde_json::to_string(&TransReq::new("content to translate", "en", "elb")).unwrap()
    }
}

#[derive(Serialize, Deserialize)]
pub struct TransResponse {
    pub content: String,
    pub able_to_translate: bool,
}

impl TransResponse {
    pub fn from(content: String, able_to_translate: bool) -> TransResponse {
        TransResponse {
            content,
            able_to_translate,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct TransErr {
    pub content: String,
}

impl TransErr {
    pub fn new(content: &str) -> TransErr {
        TransErr {
            content: content.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct JsonErr {
    pub content: String,
}

impl JsonErr {
    pub fn new(content: &str) -> JsonErr {
        JsonErr {
            content: content.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct NewTransReq {
    pub from_lang: String,
    pub to_lang: String,
    pub word: String,
    pub meanings: Vec<String>,
}

impl NewTransReq {
    pub fn new(from_lang: &str, to_lang: &str, word: &str, meanings: Vec<&str>) -> NewTransReq {
        NewTransReq {
            from_lang: from_lang.to_string(),
            to_lang: to_lang.to_string(),
            word: word.to_string(),
            meanings: meanings.into_iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn example() -> String {
        serde_json::to_string(&NewTransReq::new("en", "elb", "helllo", vec!["something"])).unwrap()
    }
}

#[derive(Serialize, Deserialize)]
pub struct NewTransErr {
    pub content: String,
}

impl NewTransErr {
    pub fn new(content: &str) -> NewTransErr {
        NewTransErr {
            content: content.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct TransWord {
    pub symbol_chain: Vec<char>,
    pub meanings: Vec<WordToOtherLang>,
}

impl TransWord {
    pub fn new(word: &str, meanings: Vec<&str>) -> TransWord {
        TransWord {
            symbol_chain: word.chars().collect(),
            meanings: meanings
                .into_iter()
                .map(|meaning| WordToOtherLang::new(meaning.chars().collect()))
                .collect(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct WordToOtherLang {
    pub symbol_chain: Vec<char>,
}
impl WordToOtherLang {
    pub fn new(symbol_chain: Vec<char>) -> WordToOtherLang {
        WordToOtherLang { symbol_chain }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Language {
    pub id: String,
    pub label: String,
    pub unit_split_symbols: Vec<char>,
    pub allowed_symbols: Vec<char>,
    pub words: Vec<TransWord>,
}

impl Language {
    pub fn new(
        id: &str,
        label: &str,
        unit_split_symbols: Vec<char>,
        allowed_symbols: Vec<char>,
        _words: Vec<TransWord>,
    ) -> Language {
        Language {
            id: (id.to_string()),
            label: (label.to_string()),
            unit_split_symbols: (unit_split_symbols),
            allowed_symbols: (allowed_symbols),
            words: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::dtos::{Language, TransWord};

    #[test]
    fn init_languages() {
        let english = Language::new(
            "en",
            "english",
            vec!['.'],
            vec!['a'],
            vec![TransWord::new("", vec!["", ""])],
        );
        assert_eq!(english.id, "en".to_string());
    }

    #[test]
    fn init_new_word() {
        let trans_word = TransWord::new("hello", vec!["hello"]);
        assert_eq!(
            trans_word.meanings[0].symbol_chain,
            vec!['h', 'e', 'l', 'l', 'o']
        );
    }
}
