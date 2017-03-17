use std;
use pz5;
use collada;

use pz5::ToPz5Mesh;

use super::Error;

use super::FromColladaLOD;

use super::VirtualMesh;
use super::VirtualLOD;
use super::Geometry;

pub trait FromColladaMesh:Sized{
    type LOD:FromColladaLOD<Error=Self::Error>;
    type Container:From<Self> + std::borrow::Borrow<Self>;
    type Error:From<Error>;

    fn build<F>(virtual_mesh:&VirtualMesh,build_mesh:F) -> Result<Self::Container,Self::Error>
        where
            F:FnOnce(&VirtualMesh) -> Result<Self::Container,Self::Error>
    {
        build_mesh(virtual_mesh)
    }
}
