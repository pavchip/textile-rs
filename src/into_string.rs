use std::fs::File;
use std::path::Path;
use std::io::Read;

pub trait IntoString {
    fn into_string(&self) -> String;
}

impl IntoString for &'static str {
    fn into_string(&self) -> String {
        self.to_string()
    }
}

impl IntoString for String {
    fn into_string(&self) -> String {
        self.to_owned()
    }
}

impl<'a> IntoString for &'a Path {
    fn into_string(&self) -> String {
        let mut file = File::open(self).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        contents
    }
}
