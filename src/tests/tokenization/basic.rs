use crate::tokenization::basic::*;

fn expected() -> Vec<BasicToken> {
    vec![
            BasicToken::SpecialSymbol('.'),
            BasicToken::Word(String::from("creature")),
            BasicToken::SpecialSymbol('{'),
            BasicToken::Word(String::from("max")),
            BasicToken::SpecialSymbol('-'),
            BasicToken::Word(String::from("health")),
            BasicToken::SpecialSymbol(':'),
            BasicToken::Word(String::from("100")),
            BasicToken::SpecialSymbol('.'),
            BasicToken::Word(String::from("0")),
            BasicToken::SpecialSymbol(';'),
            BasicToken::Word(String::from("max")),
            BasicToken::SpecialSymbol('-'),
            BasicToken::Word(String::from("stamina")),
            BasicToken::SpecialSymbol(':'),
            BasicToken::Word(String::from("100")),
            BasicToken::SpecialSymbol('.'),
            BasicToken::Word(String::from("0")),
            BasicToken::SpecialSymbol(';'),
            BasicToken::Word(String::from("speed")),
            BasicToken::SpecialSymbol(':'),
            BasicToken::Word(String::from("10")),
            BasicToken::SpecialSymbol('.'),
            BasicToken::Word(String::from("0")),
            BasicToken::SpecialSymbol(';'),
            BasicToken::SpecialSymbol('}'),
        ]
}

#[test]
fn spaces_test() {
    let test_str = String::from(r#"
    .creature {max-health:     100.0;max-stamina: 100.0;
        speed :10.0;
    }
"#);

    let result = BasicTokensIterator::new(test_str).collect::<Vec<BasicToken>>();
    println!("Tokenization result: ");
    for token in &result {
        println!("{:?}", token);
    }
    println!("");
    
    let expected = expected();
    assert_eq!(expected.len(), result.len());
    for i in 0..expected.len(){
        assert_eq!(expected[i], result[i]);
    }
}