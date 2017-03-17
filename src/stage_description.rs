use std;
use std::path::Path;
use StageDescriptionTrait;

pub enum StageDescription<'a>{
    Model(&Path),
    //Mesh,
}

impl StageDescriptionTrait for StageDescription{
}

impl std::fmt::Display for StageDescription{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self{
            StageDescription::Model(ref e) => write!(f, "Collada error:{}", e),
            //Error::StringParseError(ref message) => write!(f, "String parse error: {}", message),
            //Error::SemanticsParse(ref e) => write!(f, "Semantics parse error:{}", e),
            //Error::ByteOrderError(ref e) => write!(f, "Byte order error:{}", e),
            //Error::Other(ref message) => write!(f, "{}", message),
        }
    }
}
