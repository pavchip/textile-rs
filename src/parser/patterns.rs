use regex::Regex;

lazy_static! {
    pub static ref BLOCK_QUOTATION_PATTERN: Regex = Regex::new("bq(?P<attributes>.*?)(?P<mode>\\.{1,2}) ").unwrap();
    pub static ref CODE_BLOCK_PATTERN: Regex = Regex::new("bc(?P<attributes>.*?)(?P<mode>\\.{1,2}) ").unwrap();
    pub static ref COMMENT_PATTERN: Regex = Regex::new("#{3}(?P<mode>\\.{1,2}) ").unwrap();
    pub static ref HEADING_PATTERN: Regex = Regex::new("h(?P<level>[1-6])(?P<attributes>.*)\\. ").unwrap();
    pub static ref NO_TEXTILE_BLOCK_PATTERN: Regex = Regex::new("notextile(?P<mode>\\.{1,2}) ").unwrap();
    pub static ref PARAGRAPH_PATTERN: Regex = Regex::new("(?:p(?P<attributes>.*)\\. )?").unwrap();

    pub static ref ABBREVIATION_PATTERN: Regex = Regex::new(r"^(?P<abbreviation>\p{Lu}{3,})(?:\((?P<transcript>.*?)\))?").unwrap();
    pub static ref BOLD_TEXT_PATTERN: Regex = Regex::new(r"^(?P<count1>\*+)(?P<string>.+?)(?P<count2>\*+)").unwrap();
    pub static ref CITATION_PATTERN: Regex = Regex::new(r"^\?\?(?P<string>.+?)\?\?").unwrap();
    pub static ref CODE_PATTERN: Regex = Regex::new("^@(?P<code>.*?)@").unwrap();
    pub static ref IMAGE_PATTERN: Regex = Regex::new("^!(?P<align>[<|>|=]?)(?P<string>.+?)!").unwrap();
    pub static ref IMAGE_URL_ALT_PATTERN: Regex = Regex::new("(?P<url>[^\\(\\) ]+)(?:\\((?P<alt>.+)\\))?").unwrap();
    pub static ref ITALIC_TEXT_PATTERN: Regex = Regex::new("^(?P<count1>_+)(?P<string>.+?)(?P<count2>_+)").unwrap();
    pub static ref LINK_PATTERN: Regex = Regex::new("^\"(?P<string>.+?)\":(?P<url>[^ \\(\\)]+)").unwrap();
    pub static ref NO_TEXTILE_INLINE_PATTERN: Regex = Regex::new("^={2}(?P<string>.*?)={2}").unwrap();
    pub static ref SPAN_PATTERN: Regex = Regex::new("^(?P<count1>%+)(?P<string>.+?)(?P<count2>%+)").unwrap();
    pub static ref STRIKETHROUGH_TEXT_PATTERN: Regex = Regex::new("^(?P<count1>-+)(?P<string>.+?)(?P<count2>-+)").unwrap();
    pub static ref SUBSCRIPT_TEXT_PATTERN: Regex = Regex::new("^(?P<count1>~+)(?P<string>.+?)(?P<count2>~+)").unwrap();
    pub static ref SUPERSCRIPT_TEXT_PATTERN: Regex = Regex::new(r"^(?P<count1>\^+)(?P<string>.+?)(?P<count2>\^+)").unwrap();
    pub static ref UNDERLINED_TEXT_PATTERN: Regex = Regex::new(r"^(?P<count1>\++)(?P<string>.+?)(?P<count2>\++)").unwrap();

    pub static ref ATTRS_STR_PATTERN: Regex = Regex::new("(?:^([\\[\\{\\(].+?[\\]\\}\\)])*)").unwrap();
    pub static ref PADDING_PATTERN: Regex = Regex::new("\\(+|\\)+").unwrap();
    pub static ref ALIGN_PATTERN: Regex = Regex::new("[<|>|=]{1,2}").unwrap();
    pub static ref LANG_PATTERN: Regex = Regex::new("\\[([A-Za-z]{2}(?:-[A-Za-z]{2})?)\\]").unwrap();
    pub static ref CLASS_ID_PATTERN: Regex = Regex::new("\\((?P<class>[\\w-_\\. ]+)?(?:#(?P<id>[\\w-_]+))?\\)").unwrap();
    pub static ref CLASS_STR_SPLIT_PATTERN: Regex = Regex::new(" +").unwrap();
    pub static ref CSS_PROPS_PATTERN: Regex = Regex::new("\\{([^\\{\\}]+)\\}").unwrap();
    pub static ref CSS_PROPS_SPLIT_PATTERN: Regex = Regex::new("; *").unwrap();
    pub static ref CSS_PROP_STR_PATTERN: Regex = Regex::new("(?P<key>[a-z-_]+): *(?P<value>.+)").unwrap();
}
