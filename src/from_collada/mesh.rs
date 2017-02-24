use std;
use pz5;
use collada;

use std::rc::Rc;

use pz5::ToPz5Mesh;

use super::Error;

use super::FromColladaLOD;

use super::VirtualMesh;
use super::VirtualLOD;
use super::Geometry;

pub trait FromColladaMesh:Sized{
    type LOD:FromColladaLOD<Error=Self::Error>;
    type Container:From<Self::LOD> + std::borrow::Borrow<Self::LOD>;
    type Error:From<Error>;

    fn get_name(&self) -> &String;

    fn build<F>(virtual_mesh:&VirtualMesh,build_mesh:&F) -> Result<Self,Self::Error>
        where
            F:Fn(&VirtualMesh) -> Result<Self,Self::Error>
    {
        let mesh=build_mesh(virtual_mesh)?;
        Ok(mesh)
    }

    fn build_lods<F>(virtual_mesh:&VirtualMesh,build_lod:F) -> Result<Vec<Self::Container>,Self::Error>
        where
            F:Fn(&VirtualLOD,Geometry) -> Result< <Self as FromColladaMesh>::LOD,Self::Error>
    {
        let mut lods=Vec::with_capacity(virtual_mesh.lods.len());

        for virtual_lod in virtual_mesh.lods.iter(){
            let lod = <Self as FromColladaMesh>::LOD::build(virtual_lod,&build_lod)?;

            lods.push(Self::Container::from(lod));
        }

        Ok(lods)
    }
}
