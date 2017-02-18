use std;
use pz5;
use collada;
use Error;

use pz5::ToPz5Mesh;

use FromColladaLOD;

use from_collada::VirtualMesh;
use from_collada::VirtualLOD;

pub trait FromColladaMesh: ToPz5Mesh{
    type LOD:FromColladaLOD<Error=Self::Error>;
    type Error:From<Error>;

    fn build<F>(virtual_mesh:&VirtualMesh,build_mesh:&F) -> Result<Self,Self::Error>
        where
            F:Fn(&VirtualMesh) -> Result<Self,Self::Error>
    {
        let mesh=build_mesh(virtual_mesh)?;
        Ok(mesh)
    }

    fn build_lods<F>(virtual_mesh:&VirtualMesh,build_lod:F) -> Result<Vec< <Self as FromColladaMesh>::LOD>,Self::Error>
        where
            F:Fn(&VirtualLOD,Vec<u8>) -> Result< <Self as FromColladaMesh>::LOD,Self::Error>
    {
        let mut lods=Vec::with_capacity(virtual_mesh.lods.len());

        for virtual_lod in virtual_mesh.lods.iter(){
            let lod = <Self as FromColladaMesh>::LOD::build(virtual_lod,&build_lod)?;

            lods.push(lod);
        }

        Ok(lods)
    }
}
