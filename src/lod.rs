use Error;
use pz5;
use collada;
use byteorder;

use VirtualSource;
//use Semantics;
//use SemanticsSourceLayerType;
/*

pub trait LODFromCollada{
    pub fn build(virtual_lod:&VirtualLOD) -> Result<Self,Error>;
    pub fn build_geometry(virtual_lod:&VirtualLOD) -> Result<Vec<u8>,Error>{
        let mut data_size=0;
        /*
        for source in virtual_lod.sources.iter(){
            for layer in source.layers.iter(){
                let layer_type_size=match layer.layer_type{
                    SemanticsSourceLayerType::Float => 4,
                    SemanticsSourceLayerType::Int => 4,
                }
        */
        let vertices_count=virtual_lod.sources[0].vertex_layer.indexes.len();

        let mut data:Vec<u8>=Vec::with_capacity(1024);

        //vcount==0? calc size, indexes? not supported check

        for vertex_index in 0..vertices_count {
            for source in virtual_lod.sources.iter(){
                let index=source.vertex_layer.indexes[vertex_index];

                for layer in source.layers.iter(){
                    match layer.layer_type{
                        SemanticsSourceLayerType::Float => {
                            let value=match layer.layer{
                                collada::SourceLayer::Float( ref layer_data ){
                                    layer_data[index] as f32
                                }collada::SourceLayer::Integer( ref layer_data ){
                                    layer_data[index] as f32
                                }
                            };

                            match data.write_f32::<LittleEndian>(value){
                                Ok ( _ ) => {},
                                Err( e ) => return Err( Error::ByteOrderError(e) ),
                            }
                        },
                        SemanticsSourceLayerType::Int => {
                            let value=match layer.layer{
                                collada::SourceLayer::Float( ref layer_data ){
                                    layer_data[index] as i32
                                }collada::SourceLayer::Integer( ref layer_data ){
                                    layer_data[index] as i32
                                }
                            };

                            match data.write_i32::<LittleEndian>(value){
                                Ok ( _ ) => {},
                                Err( e ) => return Err( Error::ByteOrderError(e) ),
                            }
                        },
                    }
                }
            }
        }

        Ok(data)
    }
}
*/

pub struct VirtualLOD<'a>{
    pub distance:f32,
    pub sources:Vec<VirtualSource<'a>>,
}

impl<'a> VirtualLOD<'a>{
    pub fn construct(collada_mesh:&'a collada::Mesh, distance:f32, semantics_text:&String) -> Result<VirtualLOD<'a>,Error>{
        let semantics=pz5::Semantics::parse(semantics_text).unwrap();//?;

        let mut sources=Vec::new();

        for semantics_source in semantics.sources.iter(){
            let virtual_source=VirtualSource::construct(collada_mesh, semantics_source)?;

            sources.push(virtual_source);
        }

        Ok(
            VirtualLOD{
                distance:distance,
                sources:sources,
            }
        )
    }
}
