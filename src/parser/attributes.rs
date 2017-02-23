use super::Attributes;
use super::patterns::{
    ATTRS_STR_PATTERN,
    PADDING_PATTERN,
    ALIGN_PATTERN,
    LANG_PATTERN,
    CLASS_ID_PATTERN,
    CSS_PROPS_PATTERN,
    CSS_PROPS_SPLIT_PATTERN,
    CSS_PROP_STR_PATTERN
};

pub fn parse_block_attributes(attrs_str: &str) -> Attributes {
    let (mut attrs, string, mut css_props) = parse_attributes(attrs_str);

    if PADDING_PATTERN.is_match(&*string) {
        let caps = PADDING_PATTERN.captures(&*string).unwrap();
        let padding = caps.at(0).unwrap();
        let padding_type = if padding.starts_with("(") {
            "padding-left"
        } else {
            "padding-right"
        }.to_string();
        css_props.push(format!("{}: {}em", padding_type, padding.len()));
    }

    if ALIGN_PATTERN.is_match(&*string) {
        let caps = ALIGN_PATTERN.captures(&*string).unwrap();
        let text_align = match caps.at(0).unwrap() {
            ">" => "right",
            "<>" => "justify",
            "=" => "center",
            _ => "left",
        }.to_string();
        css_props.push(format!("text-align: {}", text_align));
    }

    if !css_props.is_empty() {
        attrs.insert("style".to_string(), css_props.join("; "));
    }

    attrs
}

pub fn parse_inline_attributes(text: &str) -> (Attributes, String) {
    // Match any brackets at the beginning of string.
    let attrs_string = ATTRS_STR_PATTERN.captures(text).unwrap().at(0).unwrap();
    let (mut attrs, _, css_props) = parse_attributes(attrs_string);

    if text.starts_with(attrs_string) {
        if !css_props.is_empty() {
            attrs.insert("style".to_string(), css_props.join("; "));
        }

        (attrs, text.replace(attrs_string, ""))
    } else {
        (Attributes::new(), text.to_string())
    }
}

fn parse_attributes(attrs_str: &str) -> (Attributes, String, Vec<String>) {
    let mut attrs = Attributes::new();
    let mut string = attrs_str.to_string();
    let mut css_props = Vec::new();

    if LANG_PATTERN.is_match(attrs_str) {
        let lang = LANG_PATTERN.captures(attrs_str).unwrap().at(1).unwrap();
        string = string.replace(lang, "");
        attrs.insert("lang".to_string(), lang.to_string());
    }

    if CLASS_ID_PATTERN.is_match(attrs_str) {
        let caps = CLASS_ID_PATTERN.captures(attrs_str).unwrap();
        let class = caps.name("class").unwrap_or("").to_string();
        let id = caps.name("id").unwrap_or("").to_string();

        if !class.is_empty() {
            attrs.insert("class".to_string(), class);
        }

        if !id.is_empty() {
            attrs.insert("id".to_string(), id);
        }
        string = string.replace(caps.at(0).unwrap(), "");
    }

    if CSS_PROPS_PATTERN.is_match(attrs_str) {
        let caps = CSS_PROPS_PATTERN.captures(attrs_str).unwrap();

        for css_prop in CSS_PROPS_SPLIT_PATTERN.split(caps.at(1).unwrap()) {
            let caps = CSS_PROP_STR_PATTERN.captures(css_prop).unwrap();
            let key = caps.name("key").unwrap().to_string();
            let value = caps.name("value").unwrap().to_string();
            css_props.push(format!("{}: {}", key, value));
        }
        string = string.replace(caps.at(0).unwrap(), "");
    }

    (attrs, string, css_props)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_block_attributes_correctly() {
        assert_eq!(
            parse_block_attributes("<>)))){color: black}(my-class)"),
            hashmap!{
                "class".to_string() => "my-class".to_string(),
                "style".to_string() => "color: black; padding-right: 4em; text-align: justify".to_string(),
            }
        );
    }

    #[test]
    fn parses_inline_attributes_correctly() {
        assert_eq!(
            parse_inline_attributes("(class another-class#id)[en]{font-size: 1em; background-color: #fff}"),
            (
                hashmap!{
                    "class".to_string() => "class another-class".to_string(),
                    "id".to_string() => "id".to_string(),
                    "lang".to_string() => "en".to_string(),
                    "style".to_string() => "font-size: 1em; background-color: #fff".to_string(),
                },
                "".to_string(),
            )
        );
        assert_eq!(
            parse_inline_attributes("{text-align: center;}(class-name)"),
            (
                hashmap!{
                    "class".to_string() => "class-name".to_string(),
                    "style".to_string() => "text-align: center".to_string(),
                },
                "".to_string(),
            )
        );
    }
}
