extern crate pyramid;
extern crate pyramid_animation;
extern crate pyramid_viewport;
extern crate pyramid_template;
extern crate pyramid_asset3d;
extern crate pyramid_transform;

use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

use pyramid::document::*;
use pyramid::system::*;

fn main() {

    let path = Path::new("../examples/levela/map.pml");
    let root_path = path.parent().unwrap().to_path_buf();
    let mut system = System::new();
    system.add_subsystem(Box::new(pyramid_asset3d::Asset3dSubSystem::new(root_path.clone())));
    system.add_subsystem(Box::new(pyramid_template::TemplateSubSystem::new(root_path.clone())));
    system.add_subsystem(Box::new(pyramid_animation::AnimationSubSystem::new()));
    system.add_subsystem(Box::new(pyramid_transform::TransformSubSystem::new()));
    system.add_subsystem(Box::new(pyramid_viewport::ViewportSubSystem::new(root_path.clone())));

    let doc = Document::from_file(path).unwrap();
    system.set_document(doc);

    println!("Starting main loop");
    while system.running {
        system.update();
    }
    let mut f = File::create("doc_state.xml").unwrap();
    f.write_all(&system.document().to_string().into_bytes()).unwrap();
}
