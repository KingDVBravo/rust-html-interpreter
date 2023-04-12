pub enum TagType {
    HEAD,
    BODY,
    H1,
    H2,
    H3,
    H4,
    H5,
    P,
    EOF,
}
pub struct Tag {
    tagtype : TagType,
}
pub fn assign_tag_type(input : String, tag : Tag) -> TagType{
    //for words : input.split(pat)
    return TagType::EOF;
}

fn main() {
    println!("Hello, world!");
}
