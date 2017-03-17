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
    type Container:From<Self> + std::borrow::Borrow<Self>;
    type Error:From<Error>;

    fn build<F>(file_name:&Path,build_model:F) -> Result<Self::Container,Self::Error>
        where
            F:FnOnce(&collada::Document,&HashMap<String,VirtualMesh>) -> Result<Self::Container,Self::Error>
    {
        let document=VirtualModel::parse_collada(file_name)?;

        let virtual_meshes=VirtualModel::generate_virtual_meshes(&document)?;

        build_model(&document,&virtual_meshes)
    }
}
