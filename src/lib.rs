use crfs::{Attribute, Model};
use regex::Regex;
use std::vec;

pub fn text_tagger(input_str: &String) -> Vec<(&str, String)> {
    let grapheme_raw_regex =
        r"([\u1780-\u17FF](\u17d2[\u1780-\u17FF]|[\u17B6-\u17D1\u17D3\u17DD])*)";
    let grapheme_regex: Regex = Regex::new(grapheme_raw_regex).unwrap();
    let characters_regex = Regex::new(
        r"([\u1780-\u17d3]+)|([\u17d4-\u17dd]+)|([\u17e0-\u17e9]+)|(\s+)|([^\u1780-\u17ff\s]+)",
    )
    .unwrap();

    let mut outputs = vec![];
    for capture in characters_regex.captures_iter(&input_str) {
        let (data_type, text) = capture
            .iter()
            .enumerate()
            .skip(1)
            .find(|t| t.1.is_some())
            .map(|t| (t.0, t.1.unwrap().as_str()))
            .unwrap();

        if data_type != 1 {
            outputs.push((text, "NS".to_string()));
            continue;
        }

        for match_item in grapheme_regex.find_iter(text) {
            let grapheme = match_item.as_str();
            let size = grapheme.chars().count();
            let grapheme_type: String = if size == 1 {
                "C".to_string()
            } else {
                format!("K{}", size)
            };

            outputs.push((grapheme, grapheme_type));
        }
    }

    return outputs;
}

pub fn create_features(kccs: &Vec<(&str, String)>) -> Vec<Vec<Attribute>> {
    let mut chunks: Vec<Vec<Attribute>> = vec![];
    let size = kccs.len();

    for (i, el) in kccs.iter().enumerate() {
        // initial feature
        let mut items = vec![
            Attribute::new(format!("kcc:{}", el.0), 1.0),
            Attribute::new(format!("t:{}", el.1), 1.0),
            Attribute::new("ns".to_string(), if el.1 == "NS" { 1.0 } else { 0.0 }),
        ];

        // bos
        if i == 0 {
            items.push(Attribute::new("BOS", 1.0));
        }

        if i >= 1 {
            items.push(Attribute::new(format!("kcc[-1]:{}", kccs[i - 1].0), 1.0));
            items.push(Attribute::new(format!("kcc[-1]t:{}", kccs[i - 1].1), 1.0));
            items.push(Attribute::new(
                format!("kcc[-1:0]:{}{}", kccs[i - 1].0, el.0),
                1.0,
            ));
            let ns_1 = if kccs[i - 1].1 == "NS" { 1.0 } else { 0.0 };
            items.push(Attribute::new("ns-1".to_string(), ns_1));
        }

        if i >= 2 {
            items.push(Attribute::new(format!("kcc[-2]:{}", kccs[i - 2].0), 1.0));
            items.push(Attribute::new(format!("kcc[-2]t:{}", kccs[i - 2].1), 1.0));
            items.push(Attribute::new(
                format!("kcc[-2:-1]:{}{}", kccs[i - 2].0, kccs[i - 1].0),
                1.0,
            ));
            items.push(Attribute::new(
                format!("kcc[-2:0]:{}{}{}", kccs[i - 2].0, kccs[i - 1].0, kccs[i].0,),
                1.0,
            ));
        }

        if i >= 3 {
            items.push(Attribute::new(format!("kcc[-3]:{}", kccs[i - 3].0), 1.0));
            items.push(Attribute::new(format!("kcc[-3]t:{}", kccs[i - 3].1), 1.0));
            items.push(Attribute::new(
                format!(
                    "kcc[-3:0]:{}{}{}{}",
                    kccs[i - 3].0,
                    kccs[i - 2].0,
                    kccs[i - 1].0,
                    kccs[i].0
                ),
                1.0,
            ));
            items.push(Attribute::new(
                format!(
                    "kcc[-3:-1]:{}{}{}",
                    kccs[i - 3].0,
                    kccs[i - 2].0,
                    kccs[i - 1].0
                ),
                1.0,
            ));
            items.push(Attribute::new(
                format!("kcc[-3:-2]:{}{}", kccs[i - 3].0, kccs[i - 2].0),
                1.0,
            ));
        }

        if size >= 1 && i < size - 1 {
            items.push(Attribute::new(format!("kcc[+1]:{}", kccs[i + 1].0), 1.0));
            items.push(Attribute::new(format!("kcc[+1]t:{}", kccs[i + 1].1), 1.0));
            items.push(Attribute::new(
                format!("kcc[+1:0]t:{}{}", kccs[i].0, kccs[i + 1].0),
                1.0,
            ));
            items.push(Attribute::new(
                format!("ns+1"),
                if kccs[i + 1].1 == "NS" { 1.0 } else { 0.0 },
            ));
        }

        if size >= 2 && i < size - 2 {
            items.push(Attribute::new(format!("kcc[+2]:{}", kccs[i + 2].0), 1.0));
            items.push(Attribute::new(format!("kcc[+2]t:{}", kccs[i + 2].1), 1.0));
            items.push(Attribute::new(
                format!("kcc[+1:+2]:{}{}", kccs[i + 1].0, kccs[i + 2].0),
                1.0,
            ));
            items.push(Attribute::new(
                format!("kcc[0:+2]:{}{}{}", kccs[i].0, kccs[i + 1].0, kccs[i + 2].0),
                1.0,
            ));
            items.push(Attribute::new(
                "ns+2",
                if kccs[i + 2].1 == "NS" { 1.0 } else { 0.0 },
            ));
        }

        if size >= 3 && i < size - 3 {
            items.push(Attribute::new(format!("kcc[+3]:{}", kccs[i + 3].0), 1.0));
            items.push(Attribute::new(format!("kcc[+3]t:{}", kccs[i + 3].1), 1.0));
            items.push(Attribute::new(
                format!("kcc[+2:+3]t:{}{}", kccs[i + 2].0, kccs[i + 3].0),
                1.0,
            ));
            items.push(Attribute::new(
                format!(
                    "kcc[+1:+3]t:{}{}{}",
                    kccs[i + 1].0,
                    kccs[i + 2].0,
                    kccs[i + 3].0
                ),
                1.0,
            ));
            items.push(Attribute::new(
                format!(
                    "kcc[0:+3]t:{}{}{}{}",
                    kccs[i].0,
                    kccs[i + 1].0,
                    kccs[i + 2].0,
                    kccs[i + 3].0
                ),
                1.0,
            ));
        }

        if size >= 1 && i == size - 1 {
            items.push(Attribute::new("EOS", 1.0));
        }

        chunks.push(items);
    }

    return chunks;
}

pub fn tokenize(model: &Model, input_str: &String) -> Vec<String> {
    let normalized_text = input_str.replace("\u{200b}", "");
    let graphemes = text_tagger(&normalized_text);
    let features = create_features(&graphemes);
    let mut tagger = model.tagger().unwrap();
    let results: Vec<&str> = tagger.tag(&features).unwrap();
    let mut tokens = vec![];

    
    for (i, y) in results.iter().enumerate() {
        let (c, _) = graphemes.get(i).unwrap();
        let flag = y.parse::<i8>().unwrap();
        if flag == 1 || i == 0 {
            tokens.push(c.to_string());
            continue;
        }
        tokens.last_mut().unwrap().push_str(c);
    }
    tokens
}
