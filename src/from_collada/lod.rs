use std;
use pz5;
use collada;

use std::rc::Rc;

use pz5::ToPz5LOD;

use super::Error;
use super::VirtualLOD;
use super::Geometry;


pub trait FromColladaLOD:Sized{
    type Error:From<Error>;

    fn build<F>(virtual_lod:&VirtualLOD,build_lod:&F) -> Result<Self,Self::Error>
        where
            F:Fn(&VirtualLOD,Geometry) -> Result<Self,Self::Error>
    {
        let lod=build_lod(virtual_lod, Geometry::new(virtual_lod.geometry.clone()) )?;

        Ok(lod)
    }

}
