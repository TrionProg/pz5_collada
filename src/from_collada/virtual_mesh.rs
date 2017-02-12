use std;
use pz5;
use collada;
use Error;


/*
pub struct Mesh<V>{
    //material:String,
    name:String,
    geometry_type:GeometryType,
    lods:Vec<LOD<V>>,
    //geometryType
    //vertexFormat:String,
    //lods:Lod<V>,
}*/
/*
pub trait Mesh{
    fn read_layer(virtual_mesh_lod:&collada::Mesh, cursor:&mut Cursor) -> Result<(&collada::VertexLayer,bool,Vec<&SourceLayer>), Error>{
        let vertex_layer=match cursor.next(){
            Lexeme::EOF => break,
            Lexeme::String(ref layerName) => {
                let vertex_layer=match mesh.get(layerName){
                    Some(ref vl) => vl,
                    None => return Err( Error::Other(format!("Mesh \"{}\" has no vertex layer \"{}\"",&virtual_mesh_lod.name,layerName)) )б
                };

                vertex_layer
            },
            _=>return Err( Error::SemanticsParse(format!("Expected name of layer, but {} has been found", cursor.lex.print())) ),
        };

        let index=match cursor.next(){
            Lexeme::Ampersand => { cursor.next(); true},
            _ => false,
        };

        if cursor.lex!=Lexeme::Bracket {
            return Err( Error::SemanticsParse(format!("Expected (, but {} has been found", cursor.lex.print())) );
        }

        let mut source_layers=Vec::new();

        loop{
            match cursor.next(){
                Lexeme::String(ref source_layer_name) =>


    }
    fn construct(virtual_mesh:&VirtualMesh, outputSemantics:&String) -> Result<Box<Mesh>, Error>{
        for cM in virtual_mesh.lods.iter(){
            let virtual_mesh=cM.unwrap();

            let mut cursor=Cursor::new(outputSemantics.as_str());

            let mut layers=Vec::new();

            loop{
                match cursor.next(){
                    Lexeme::EOF => break,
                    Lexeme::String(ref layerName) => {
                        let vertex_layer=match mesh.get(layerName){
                            Some(ref vl) => vl,
                            None => return Err( Error::Other(format!("Mesh \"{}\" has no vertex layer \"{}\"",&virtual_mesh.name,layerName)) )б
                        };

                        layers.push( (vertex_layer,
*/

//impl<V> Mesh<V>{

use from_collada::VirtualLOD;

pub struct VirtualMesh<'a>{
    pub name:String,
    pub full_semantics:String,
    pub lods:Vec<VirtualLOD<'a>>,
    pub geometry_type:pz5::GeometryType,
}

impl<'a> VirtualMesh<'a>{
    pub fn check(&self) -> Result<(),Error>{
        //TODO:check geometry_type, number of vertices etc
        Ok(())
    }
}

/*
pub struct Mesh{
    pub name:String,
    pub full_semantics:String,
    pub lods:vec<LOD>,
    pub geometry_type:GeometryType,
}
*/

//impl Mesh{
//    pub fn build(virtual_mesh:&VirtualMesh) -> Result<Mesh,Error>{
