use std;
use pz5;
use collada;
use byteorder;
use Error;

use pz5::Pz5LOD;

use from_collada::VirtualLOD;

use byteorder::LittleEndian;
use byteorder::WriteBytesExt;


pub trait FromColladaLOD: Pz5LOD{
    type Error:From<Error>;

    fn build<F>(virtual_lod:&VirtualLOD,build_lod:&F) -> Result<Self,Self::Error>
        where
            F:Fn(&VirtualLOD,Vec<u8>) -> Result<Self,Self::Error>
    {
        let data=match Self::build_geometry(virtual_lod){
            Ok ( d ) => d,
            Err( e ) => return Err(Self::Error::from(e)),
        };

        let lod=build_lod(virtual_lod,data)?;
        Ok(lod)
    }

    fn build_geometry(virtual_lod:&VirtualLOD) -> Result<Vec<u8>,Error>{
        /*
        TODO:add indexes
        TODO:add data_size)calculation
        I think, this function should return vec of vec<u8> vec[0] is geometry with indexes, vec[1] or more -data like XYZ
        IDEA:Some info like indexes may be u16 for smaller size, but reading wont be so easy
        let mut data_size=0;

        for source in virtual_lod.sources.iter(){
            for layer in source.layers.iter(){
                let layer_type_size=match layer.layer_type{
                    SemanticsSourceLayerType::Float => 4,
                    SemanticsSourceLayerType::Int => 4,
                }
            }
        }
        */

        let mut data:Vec<u8>=Vec::with_capacity(1024);

        /*
        match data.write_u64::<LittleEndian>(vertices_count as u64){ //data.len
            Ok ( _ ) => {},
            Err( e ) => return Err( Error::ByteOrderError(e) ),
        }
        */

        for vertex_index in 0..virtual_lod.vertices_count {
            for source in virtual_lod.sources.iter(){
                let index=source.vertex_layer.indexes[vertex_index];

                for layer in source.layers.iter(){
                    match layer.layer_type{
                        pz5::SemanticsSourceLayerType::Float => {
                            let value=match *layer.layer{
                                collada::SourceLayer::Float( ref layer_data ) => {
                                    layer_data[index] as f32
                                }collada::SourceLayer::Integer( ref layer_data ) => {
                                    layer_data[index] as f32
                                }
                            };

                            match data.write_f32::<LittleEndian>(value){
                                Ok ( _ ) => {},
                                Err( e ) => return Err( Error::ByteOrderError(e) ),
                            }
                        },
                        pz5::SemanticsSourceLayerType::Int => {
                            let value=match *layer.layer{
                                collada::SourceLayer::Float( ref layer_data ) => {
                                    layer_data[index] as i32
                                }collada::SourceLayer::Integer( ref layer_data ) => {
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