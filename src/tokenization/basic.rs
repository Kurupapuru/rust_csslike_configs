use std::vec::IntoIter;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum BasicToken {
    Word(String),
    SpecialSymbol(char)
}

pub(crate) struct BasicTokensIterator {
    chars: IntoIter<char>,
    str_builder: string_builder::Builder,
    remaining_token: Option<BasicToken>
}

impl BasicTokensIterator {
    pub(crate) fn new(str: String) -> BasicTokensIterator {
        BasicTokensIterator {
            chars: str.chars().collect::<Vec<char>>().into_iter(),
            str_builder: string_builder::Builder::default(),
            remaining_token: None
        }
    }

    fn get_string_and_clear(&mut self) -> String{
        let mut str_builder = string_builder::Builder::default();
        std::mem::swap(&mut self.str_builder, &mut str_builder);
        return str_builder.string().unwrap();
    }
}

impl Iterator for BasicTokensIterator {
    type Item = BasicToken;

    fn next(&mut self) -> Option<Self::Item> {
        // if we have cached token return it and set property to None
        if self.remaining_token.is_some() {
            let mut remaining_token = None;
            std::mem::swap(&mut remaining_token, &mut self.remaining_token);
            return remaining_token;
        }

        loop {
            let char = self.chars.next();
            if char.is_none(){
                // stop iteration if encountered end of file
                break;
            }
            let char = char.unwrap();
            let char = char.to_ascii_lowercase();

            // if word char add to word token builder
            if ('a'..='z').contains(&char) || ('0'..='9').contains(&char) {
                self.str_builder.append(char);
                continue;
            }

            // if encountered space or new line return word or go to next char
            if char == ' ' || char == '\n' {
                if self.str_builder.len() == 0 {
                    continue;
                } else {
                    return Some(BasicToken::Word(self.get_string_and_clear()));
                }
            }

            // check if char is special symbol
            let simple_token : Option<BasicToken> = match char {
                '.' | '-' | '_' | '{' | '}' | ':' | ';' | '>' => Some(BasicToken::SpecialSymbol(char)),
                _ => None
            };
            if let Some(simple_token) = simple_token {
                // if we have unfinished word, cache special symbol to return in next iteration and return word
                // otherwise just return special symbol
                if self.str_builder.len() > 0 {
                    self.remaining_token = Some(simple_token);
                    return Some(BasicToken::Word(self.get_string_and_clear()));
                } else {
                    return Some(simple_token);
                }
            }

            // panic if encountered unknown symbol
            panic!("Unknown char {}", char);
        }

        // after encountering end of file return unfinished word if present
        // otherwise stop iterating
        if self.str_builder.len() > 0 {
            return Some(BasicToken::Word(self.get_string_and_clear()))
        } else {
            return None;
        }
    }
}