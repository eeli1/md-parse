use md_parse::{parse, Markdown, Text};

#[test]
fn heading() {
    let md = "# H1\n## H2\n### H3\n#### H4\n##### H5\n###### H6\nthis is ## not a hedder ";
    assert_eq!(
        parse(md),
        Ok(vec![
            Markdown::Hedder1(vec![Text::NoFormat("H1".to_string()),]),
            Markdown::Hedder2(vec![Text::NoFormat("H2".to_string()),]),
            Markdown::Hedder3(vec![Text::NoFormat("H3".to_string()),]),
            Markdown::Hedder4(vec![Text::NoFormat("H4".to_string()),]),
            Markdown::Hedder5(vec![Text::NoFormat("H5".to_string()),]),
            Markdown::Hedder6(vec![Text::NoFormat("H6".to_string()),]),
            Markdown::Paragraph(vec![Text::NoFormat("this is ## not a hedder ".to_string())])
        ])
    );
}

#[test]
fn ordered_list() {
    let md = "1. First item\n2. Second item\n3. Third item\n3002934. 3002934\n0. zero\n43 . just a number";
    assert_eq!(
        parse(md),
        Ok(vec![
            Markdown::OrderedList(vec![Text::NoFormat("First item".to_string())], 1),
            Markdown::OrderedList(vec![Text::NoFormat("Second item".to_string())], 2),
            Markdown::OrderedList(vec![Text::NoFormat("Third item".to_string())], 3),
            Markdown::OrderedList(vec![Text::NoFormat("3002934".to_string())], 3002934),
            Markdown::OrderedList(vec![Text::NoFormat("zero".to_string())], 0),
            Markdown::Paragraph(vec![Text::NoFormat("43 . just a number".to_string())]),
        ])
    );
}
