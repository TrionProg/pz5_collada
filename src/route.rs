use std;

pub trait StageDescriptionTrait:Clone + std::fmt::Display{
}

pub struct Stage<'a, SD:StageDescriptionTrait+'a>{
    previous:Option<&'a Stage<'a,SD>>,
    description:SD,
}

impl<'a, SD:StageDescriptionTrait> Stage<'a, SD>{
    pub fn new(description:SD) -> Stage<'a,SD>{
        Stage{
            previous:None,
            description:description,
        }
    }

    pub fn new_after(after:&'a Stage<'a, SD>, description:SD) -> Stage<'a,SD>{
        Stage{
            previous:Some(after),
            description:description,
        }
    }
}



pub struct Route<SD:StageDescriptionTrait>{
    stages:Vec<SD>,
}

impl<SD:StageDescriptionTrait> Route<SD>{
    pub fn from(stage:&Stage<SD>) -> Route<SD>{
        let stages_refs={
            let mut current_stage=stage;
            let mut stages_refs=Vec::new();

            loop{
                stages_refs.push(current_stage);

                match current_stage.previous {
                    Some( ref previous ) => current_stage=previous,
                    None => break,
                }
            }

            stages_refs
        };

        let mut stages=Vec::with_capacity(stages_refs.len());

        for stage_ref in stages_refs.iter(){
            stages.push(stage_ref.description.clone());
        }

        Route{
            stages:stages,
        }
    }
}

impl<SD:StageDescriptionTrait> std::fmt::Display for Route<SD>{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.stages.iter().next(){
            Some( stage ) => write!(f, "{}", stage)?,
            None => unreachable!(),
        }

        for stage in self.stages.iter().skip(1){
            write!(f, " > {}", stage)?;
        }

        Ok(())
    }
}


/*
struct Trace<'a>{
   parent:Option<&'a Trace<'a>>,
   description:String,
}

impl<'a> Trace<'a>{
   pub fn new(description:&'static str) -> Trace{
        Trace{
            parent:None,
            description:String::from(description),
        }
    }

    pub fn push(&'a self,description:&'static str) -> Trace<'a>{
       Trace{
           parent:Some(self),
           description:String::from(description),
       }
   }
}
*/
