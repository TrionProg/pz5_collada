use Error;
use pz5;
use collada;
use std::path::Path;
//use {Stage, StageDescription};

use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::rc::Rc;

use from_collada::VirtualMesh;
use from_collada::VirtualLOD;

pub struct VirtualModel;

impl VirtualModel{
    pub fn parse_collada(file_name:&Path) -> Result<collada::Document,Error>{
        match collada::Document::parse(file_name){
            Ok( d ) => Ok(d),
            Err(e) => Err(Error::ColladaError(e)),
        }
    }

    pub fn generate_virtual_meshes<'a>(document:&'a collada::Document) -> Result<HashMap<String,VirtualMesh<'a>>,Error>{
        let (scene_name,scene)=match document.scenes.iter().next(){
            Some( (scene_name,scene) ) => (scene_name,scene),
            None => return Err(Error::Other( String::from("Collada document has no scenes") )),
        };

        let mut virtual_meshes=HashMap::new();

        for (_, node) in scene.geometries.iter(){
            let geometry=&node.joined;

            let (node_name, distance)=match node.name.find("_d_"){
                Some( pos ) => {
                    let (node_name, wast_and_distance)=node.name.split_at(pos);
                    let (wast,distance_str)=wast_and_distance.split_at("_d_".len());

                    let distance=match distance_str.parse::<f32>(){
                        Ok(d) => d,
                        Err(_) => return Err(Error::StringParseError( format!("Can not parse {} as f32",distance_str) )),
                    };

                    (String::from(node_name), distance)
                },
                None =>
                    (node.name.clone(),0.0),
            };

            for (i,mesh) in geometry.meshes.iter().enumerate(){
                let mesh_name=if geometry.meshes.len()==1 {
                    node_name.clone()
                }else{
                    match mesh.material{
                        Some( ref material_id ) =>
                            format!("{}_{}",node_name,material_id),
                        None =>
                            format!("{} #{}",node_name, i),
                    }
                };

                let smt=String::from("VERTEX:(X:float,Z:float,Y:float) NORMAL:(X:float,Y:float,Z:float)");
                let semantics=pz5::Semantics::parse(&smt).unwrap();
                //TODO:When and where use semantics from config, maybe mesh info should need

                let virtual_lod=VirtualLOD::construct(&mesh, distance, &semantics)?;

                match virtual_meshes.entry(mesh_name.clone()){
                    Entry::Vacant(entry) => {
                        let geometry_type=virtual_lod.geometry_type;

                        let mut lods=Vec::with_capacity(1);
                        lods.push(virtual_lod);

                        entry.insert(
                            VirtualMesh{
                                name:mesh_name,
                                full_semantics:smt.clone(),
                                lods:lods,
                                geometry_type:geometry_type,
                            }
                        );
                    },
                    Entry::Occupied(mut entry) =>
                        entry.get_mut().lods.push(virtual_lod),
                }
            }
        }

        for (_,virtual_mesh) in virtual_meshes.iter_mut(){
            virtual_mesh.lods.sort_by(|lod1,lod2| lod1.distance.partial_cmp(&lod2.distance).unwrap());

            if virtual_mesh.lods[0].distance!=0.0 {
                return Err( Error::Other( format!("Mesh \"{}\" must have LOD with 0 distance", virtual_mesh.name) ) );
            }

            for (lod1,lod2) in virtual_mesh.lods.iter().zip(virtual_mesh.lods.iter().skip(1)){
                if lod1.distance==lod2.distance {
                    return Err( Error::Other( format!("Mesh \"{}\" has LODS with same distance", virtual_mesh.name) ) );
                }
            }
        }

        for (_,virtual_mesh) in virtual_meshes.iter(){
            virtual_mesh.check()?;
        }

        Ok(virtual_meshes)
    }
}
