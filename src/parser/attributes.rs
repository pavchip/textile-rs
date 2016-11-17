use super::{Attributes, Attribute};
use super::patterns::{
    ATTRS_STR_PATTERN,
    PADDING_PATTERN,
    ALIGN_PATTERN,
    LANG_PATTERN,
    CLASS_ID_PATTERN,
    CLASS_STR_SPLIT_PATTERN,
    CSS_PROPS_PATTERN,
    CSS_PROPS_SPLIT_PATTERN,
    CSS_PROP_STR_PATTERN
};
use std::collections::HashMap;

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
        let padding_value = padding.len().to_string() + "em";
        css_props.insert(padding_type, padding_value);
    }

    if ALIGN_PATTERN.is_match(&*string) {
        let caps = ALIGN_PATTERN.captures(&*string).unwrap();
        let group_0 = caps.at(0).unwrap();
        let text_align = match group_0 {
            ">" => "right",
            "<>" => "justify",
            "=" => "center",
            _ => "left",
        }.to_string();
        css_props.insert("text-align".to_string(), text_align);
    }
    if !css_props.is_empty() {
        attrs.push(Attribute::Style(css_props));
    }
    attrs
}

pub fn parse_inline_attributes(text: &str) -> (Attributes, String) {
    // Match any brackets at the beginning of string.
    let attrs_string = ATTRS_STR_PATTERN.captures(text).unwrap().at(0).unwrap();
    let (mut attrs, _, css_props) = parse_attributes(attrs_string);

    if !css_props.is_empty() {
        attrs.push(Attribute::Style(css_props));
    }
    if text.starts_with(attrs_string) {
        (attrs, text.replace(attrs_string, ""))
    } else {
        (Attributes::new(), text.to_string())
    }
}

fn parse_attributes(attrs_str: &str) -> (Attributes, String, HashMap<String, String>) {
    let mut attrs = Attributes::new();
    let mut string = attrs_str.to_string();
    let mut css_props: HashMap<String, String> = HashMap::new();

    if LANG_PATTERN.is_match(attrs_str) {
        let lang = LANG_PATTERN.captures(attrs_str).unwrap().at(1).unwrap();
        string = string.replace(lang, "");
        attrs.push(Attribute::Language(lang.to_string()));
    }

    if CLASS_ID_PATTERN.is_match(attrs_str) {
        let caps = CLASS_ID_PATTERN.captures(attrs_str).unwrap();
        let class = match caps.name("class") {
            Some(value) => value,
            None => "",
        };
        let id = match caps.name("id") {
            Some(value) => value,
            None => "",
        }.to_string();

        if !class.is_empty() {
            attrs.push(Attribute::Class(CLASS_STR_SPLIT_PATTERN.split(class).map(|el| el.to_string()).collect::<Vec<String>>()));
        }

        if !id.is_empty() {
            attrs.push(Attribute::Id(id));
        }
        string = string.replace(caps.at(0).unwrap(), "");
    }

    if CSS_PROPS_PATTERN.is_match(attrs_str) {
        let caps = CSS_PROPS_PATTERN.captures(attrs_str).unwrap();

        for css_prop in CSS_PROPS_SPLIT_PATTERN.split(caps.at(1).unwrap()) {
            let caps = CSS_PROP_STR_PATTERN.captures(css_prop).unwrap();
            let key = caps.name("key").unwrap().to_string();
            let value = caps.name("value").unwrap().to_string();
            css_props.insert(key, value);
        }
        string = string.replace(caps.at(0).unwrap(), "");
    }
    (attrs, string, css_props)
}
