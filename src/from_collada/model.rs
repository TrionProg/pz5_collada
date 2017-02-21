use std;
use pz5;
use collada;

use std::path::Path;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

use pz5::ToPz5Model;
use pz5::ToPz5Mesh;

use super::Error;

use super::FromColladaMesh;
use super::VirtualModel;
use super::VirtualMesh;

pub trait FromColladaModel:Sized{
    type Mesh:FromColladaMesh<Error=Self::Error>;
    type Container:From<Self::Mesh> + std::borrow::Borrow<Self::Mesh>;
    type Error:From<Error>;

    fn build<F>(file_name:&Path,build_model:F) -> Result<Self,Self::Error>
        where
            F:FnOnce(&collada::Document,&HashMap<String,VirtualMesh>) -> Result<Self,Self::Error>
    {
        let document=VirtualModel::parse_collada(file_name)?;

        let virtual_meshes=VirtualModel::generate_virtual_meshes(&document)?;

        let model=build_model(&document,&virtual_meshes)?;
        Ok(model)
    }

    fn build_meshes<F>(virtual_meshes:&HashMap<String,VirtualMesh>,build_mesh:F) -> Result<HashMap<String, Self::Container>,Self::Error>
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
                Entry::Vacant( e ) => {e.insert(Self::Container::from(mesh));},
            }
        }

        Ok(meshes)
    }
}
