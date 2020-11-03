pub struct Config {
    pub span: bool,
    pub keywords: Vec<String>,
}

pub fn parse_cli(args: Vec<String>) -> Config {
    let span_string = String::from("span");
    let span = args.contains(&span_string);

    let mut keywords = args.to_vec();
    remove_element(&mut keywords, span_string);

    return Config {
        span,
        keywords,
    };
}

fn contains_term(args: Vec<String>, term: String) -> bool {
    return args.contains(&term);
}

fn remove_element(keywords: &mut Vec<String>, term: String) {
    let index = keywords.iter()
        .position(|item| *item == term);

    if index.is_some() {
        keywords.remove(index.unwrap());
    }
}
