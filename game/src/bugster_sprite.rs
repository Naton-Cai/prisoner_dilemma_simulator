use fyrox::{
    core::{reflect::prelude::*, type_traits::prelude::*, visitor::prelude::*},
    script::{ScriptContext, ScriptTrait},
};

#[derive(Visit, Reflect, Default, Debug, Clone, TypeUuidProvider, ComponentProvider)]
#[type_uuid(id = "861ee8cc-d9b8-4fb1-80e3-e2d55d4ad012")]
#[visit(optional)]

pub struct BugsterSprite {}

impl ScriptTrait for BugsterSprite {
    fn on_update(&mut self, _context: &mut ScriptContext) {}
}
