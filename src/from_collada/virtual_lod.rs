use std;
use pz5;
use collada;
use Error;

use from_collada::VirtualSource;

pub struct VirtualLOD<'a>{
    pub distance:f32,
    pub geometry_type:pz5::GeometryType,
    pub sources:Vec<VirtualSource<'a>>,
    pub vertices_count:usize,
}

impl<'a> VirtualLOD<'a>{
    pub fn construct(collada_mesh:&'a collada::Mesh, distance:f32, semantics:&pz5::Semantics) -> Result<VirtualLOD<'a>,Error>{
        let geometry_type=Self::get_geometry_type(collada_mesh)?;

        let mut sources=Vec::new();

        for semantics_source in semantics.sources.iter(){
            let virtual_source=VirtualSource::construct(collada_mesh, semantics_source)?;

            sources.push(virtual_source);
        }

        if sources.len()==0 {
            return Err( Error::Other(String::from("LOD has no sources")) );
        }

        let vertices_count=sources[0].vertex_layer.indexes.len();

        if vertices_count==0 {
            return Err( Error::Other(String::from("LOD has no vertices")) );
        }

        Ok(
            VirtualLOD{
                distance:distance,
                geometry_type:geometry_type,
                sources:sources,
                vertices_count:vertices_count,
            }
        )
    }

    fn get_geometry_type(collada_mesh:&collada::Mesh) -> Result<pz5::GeometryType,Error>{
        if collada_mesh.polygons.len()==0 {
            return Err(Error::Other( String::from(" has no polygons") ));
        }

        let vertex_count_per_polygon=collada_mesh.polygons[0].vertices_count;

        for polygon in collada_mesh.polygons.iter().skip(1){
            if polygon.vertices_count!=vertex_count_per_polygon {
                return Err( Error::Other(format!("Mesh expects {} vertices per polygon, but polygon with {} vertices has been found", vertex_count_per_polygon, polygon.vertices_count)) );
            }
        }

        match pz5::GeometryType::from_vertices_count(vertex_count_per_polygon){
            Ok( gt ) => Ok(gt),
            Err( e ) => Err(Error::Other(e)),
        }
    }
}
