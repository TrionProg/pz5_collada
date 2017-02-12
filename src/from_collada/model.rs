use std;
use pz5;
use collada;
use Error;

use pz5::Pz5Model;
use pz5::Pz5Mesh;
use FromColladaMesh;

use from_collada::VirtualModel;
use from_collada::VirtualMesh;

use std::path::Path;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

pub trait FromColladaModel: Pz5Model{
    type Mesh:FromColladaMesh<Error=Self::Error>;
    type Error:From<Error>;

    fn read_collada<F>(file_name:&Path,build_model:F) -> Result<Self,Self::Error>
        where
            F:FnOnce(&collada::Document,&HashMap<String,VirtualMesh>) -> Result<Self,Self::Error>
    {
        let document=VirtualModel::parse_collada(file_name).unwrap();//TODO:Error instead Unwrap

        let virtual_meshes=VirtualModel::generate_virtual_meshes(&document).unwrap();

        let model=build_model(&document,&virtual_meshes)?;
        Ok(model)
    }

    fn build_meshes<F>(virtual_meshes:&HashMap<String,VirtualMesh>,build_mesh:F) -> Result<HashMap<String, <Self as FromColladaModel>::Mesh>,Self::Error>
        where
            F:Fn(&VirtualMesh) -> Result< <Self as FromColladaModel>::Mesh,Self::Error>
    {
        let mut meshes=HashMap::new();

        for (_,virtual_mesh) in virtual_meshes.iter(){
            let mesh=<Self as FromColladaModel>::Mesh::build(virtual_mesh,&build_mesh)?;

            match meshes.entry(mesh.get_name().clone()){
                Entry::Occupied( _ ) => return Err(Self::Error::from(
                    Error::Other( format!("Mesh \"{}\" already exists",mesh.get_name()) )
                )),
                Entry::Vacant( e ) => {e.insert(mesh);},
            }
        }

        Ok(meshes)
    }
}
