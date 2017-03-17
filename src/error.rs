use std;

use collada;
/*
struct Error{
    route:Route,
    kind:ErrorKind,
}

impl Error{
    pub fn new(stage:&Stage, kind:ErrorKind) -> Error{
        Error{
            route:Route::from(stage),
            kind:kind,
        }
    }
}

macro_rules! throw{
    ($kind:expr) => (
        return Err(Error::new(current_stage,$kind))
    )
}
*/

#[derive(Debug)]
pub enum Error{
    ColladaError(collada::Error),
    StringParseError(String),
    SemanticsParse(String),
    ByteOrderError(std::io::Error),
    Other(String),
}

impl std::fmt::Display for Error{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self{
            Error::ColladaError(ref e) => write!(f, "Collada error:{}", e),
            Error::StringParseError(ref message) => write!(f, "String parse error: {}", message),
            Error::SemanticsParse(ref e) => write!(f, "Semantics parse error:{}", e),
            Error::ByteOrderError(ref e) => write!(f, "Byte order error:{}", e),
            Error::Other(ref message) => write!(f, "{}", message),
        }
    }
}

impl From<collada::Error> for Error {
    fn from(error:collada::Error) -> Error {
        Error::ColladaError(error)
    }
}
/*
pub trait NodeTrait:Clone{

}

struct Trace<'a,Node>{
    stack:Vec<Node<'a>>,
}

impl<'a,Node:NodeTrait> Trace<'a,Node>{
    pub fn new() -> Trace<'a,Node>{
        Trace{
            stack:Vec::with_capacity(4);
        }
    }

    pub fn add(&mut self,trace_node:Node) -> TraceGuard<'a,Node>{
        self.stack.push(trace_node);

        TraceGuard::new(self);
    }

    fn pop(&mut self){
        self.stack.pop();
    }
}

impl<'a,Node> Clone for Trace<'a,Node> {
    fn clone(&'a self) -> Trace<'a,Node> {
        let mut nodes=Vec::with_capacity(self.stack.len());

        for node in self.stack.iter(){
            nodes.push(node.clone());
        }

        Trace{
            stack:nodes,
        }
    }
}

struct TraceGuard<'a,Node:NodeTrait>{
    trace:&'a trace,
}

impl<'a,Node:NodeTrait> TraceGuard<'a,Node>{
    pub fn new(trace:&Trace) -> TraceGuard{
        TraceGuard{
            trace:trace,
        }
    }
}

impl<'a,Node:NodeTrait> Drop for TraceGuard<'a,Node>{
    fn drop(&mut self){
        trace.pop();
    }
}
*/

/*
impl From<i32> for Error {
    fn from(n:i32) -> Error {
        match n {
            -1 => Error::Gen,
            -2 => Error::NotOpen,
            -3 => Error::Wave,
            _ => Error::Unknown,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Sphinx audio device error:{}",
            match *self {
                Error::Gen => "Gen",
                Error::NotOpen => "NotOpen",
                Error::Wave => "Wave",
                Error::Unknown => "Unknown",
            }
        )
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Gen => "Gen",
            Error::NotOpen => "NotOpen",
            Error::Wave => "Wave",
            Error::Unknown => "Unknown",
        }
    }
    fn cause(&self) -> Option<&std::error::Error> { None }
}
*/
