#[derive(Debug, PartialEq, Clone)]
pub enum Text {
    NoFormat(String),
    Empty,
    Bold(String),
    Italic(String),
    Code(String),
    HorizontalRule,
    /// title, url
    Link(String, String),
    /// alt text, image
    Image(String, String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum List {
    UnorderedList(Vec<Text>),
    OrderedList(Vec<Text>,usize),
    None(Vec<Text>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Markdown {
    Hedder(List, u8),
    Blockquote(List,u8),
    Paragraph(List),
    Code(Option<String>, Vec<String>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Error {
    None,
}


/* 
pub fn parse(md: &str) -> Result<Vec<Markdown>, Error> {
    let mut md_vec = Vec::new();
    for mut line in md.split("\n") {
        if line.starts_with("###### ") {
            md_vec.push(Markdown::Hedder6(parse_text(&line[7..line.len()])?));
        } else if line.starts_with("##### ") {
            md_vec.push(Markdown::Hedder5(parse_text(&line[6..line.len()])?));
        } else if line.starts_with("#### ") {
            md_vec.push(Markdown::Hedder4(parse_text(&line[5..line.len()])?));
        } else if line.starts_with("### ") {
            md_vec.push(Markdown::Hedder3(parse_text(&line[4..line.len()])?));
        } else if line.starts_with("## ") {
            md_vec.push(Markdown::Hedder2(parse_text(&line[3..line.len()])?));
        } else if line.starts_with("# ") {
            md_vec.push(Markdown::Hedder1(parse_text(&line[2..line.len()])?));
        } else {
            while line.starts_with(" ") {
                line = &line[1..line.len()];
            }
            if line.len() == 0 {
                Markdown::Paragraph(vec![Text::Empty]);
                continue;
            }
            let spilt = line.split(". ");

            if let Some(num) = spilt.into_iter().next() {
                let len = num.len() + 2;
                if let Ok(num) = num.parse::<usize>() {
                    md_vec.push(Markdown::OrderedList(
                        parse_text(&line[len..line.len()])?,
                        num,
                    ));
                    continue;
                }
            }
            md_vec.push(Markdown::Paragraph(parse_text(line)?));
        }
    }
    Ok(md_vec)
}
*/

fn parse_text(line: &str) -> Result<Vec<Text>, Error> {
    let mut text = Vec::new();
    text.push(Text::NoFormat(line.to_string()));
    Ok(text)
}
