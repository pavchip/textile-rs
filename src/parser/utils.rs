use super::{Attributes, Attribute};
use regex::Regex;
use std::collections::HashMap;

pub fn parse_block_attributes(attrs_str: &str) -> Attributes {
    let (mut attrs, string, mut css_props) = parse_attributes(attrs_str);
    let padding_pattern = Regex::new("\\(+|\\)+").unwrap();
    let align_pattern = Regex::new("[<|>|=]{1,2}").unwrap();

    if padding_pattern.is_match(&*string) {
        let caps = padding_pattern.captures(&*string).unwrap();
        let padding = caps.at(0).unwrap();
        let padding_type = if padding.starts_with("(") {
            "padding-left"
        } else {
            "padding-right"
        }.to_string();
        let padding_value = padding.len().to_string() + "em";
        css_props.insert(padding_type, padding_value);
    }

    if align_pattern.is_match(&*string) {
        let caps = align_pattern.captures(&*string).unwrap();
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
    let pattern = Regex::new("(?:^([\\[\\{\\(].+?[\\]\\}\\)])*)").unwrap();
    let attrs_string = pattern.captures(text).unwrap().at(0).unwrap();
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
    let lang_pattern = Regex::new("\\[([A-Za-z]{2}(?:-[A-Za-z]{2})?)\\]").unwrap();
    let class_id_pattern = Regex::new("\\((?P<class>[\\w-_\\. ]+)?(?:#(?P<id>[\\w-_]+))?\\)").unwrap();
    let css_props_pattern = Regex::new("\\{([^\\{\\}]+)\\}").unwrap();

    if lang_pattern.is_match(attrs_str) {
        let lang = lang_pattern.captures(attrs_str).unwrap().at(1).unwrap();
        string = string.replace(lang, "");
        attrs.push(Attribute::Language(lang.to_string()));
    }

    if class_id_pattern.is_match(attrs_str) {
        let caps = class_id_pattern.captures(attrs_str).unwrap();
        let class = match caps.name("class") {
            Some(value) => value,
            None => "",
        };
        let id = match caps.name("id") {
            Some(value) => value,
            None => "",
        }.to_string();

        if !class.is_empty() {
            attrs.push(Attribute::Class(Regex::new(" +").unwrap().split(class).map(|el| el.to_string()).collect::<Vec<String>>()));
        }

        if !id.is_empty() {
            attrs.push(Attribute::Id(id));
        }
        string = string.replace(caps.at(0).unwrap(), "");
    }

    if css_props_pattern.is_match(attrs_str) {
        let caps = css_props_pattern.captures(attrs_str).unwrap();
        let css_props_split_pattern = Regex::new("; *").unwrap();
        let css_prop_pattern = Regex::new("(?P<key>[a-z-_]+): *(?P<value>.+)").unwrap();

        for css_prop in css_props_split_pattern.split(caps.at(1).unwrap()) {
            let caps = css_prop_pattern.captures(css_prop).unwrap();
            let key = caps.name("key").unwrap().to_string();
            let value = caps.name("value").unwrap().to_string();
            css_props.insert(key, value);
        }
        string = string.replace(caps.at(0).unwrap(), "");
    }
    (attrs, string, css_props)
}
