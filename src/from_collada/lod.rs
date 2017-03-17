use std;
use pz5;
use collada;

use pz5::ToPz5LOD;

use super::Error;
use super::VirtualLOD;
use super::Geometry;


pub trait FromColladaLOD:Sized{
    type Error:From<Error>;
    type Container:From<Self> + std::borrow::Borrow<Self>;

    fn build<F>(virtual_lod:&VirtualLOD,build_lod:F) -> Result<Self::Container,Self::Error>
        where
            F:FnOnce(&VirtualLOD,Geometry) -> Result<Self::Container,Self::Error>
    {
        build_lod(virtual_lod, Geometry::new(virtual_lod.geometry.clone()) )
    }

}
