extern crate pyramid;
extern crate pyramid_animation;
extern crate pyramid_viewport;
extern crate pyramid_template;

use std::path::Path;

use pyramid::document::*;
use pyramid::system::*;

fn main() {

    let path = Path::new("../examples/dml_level_a.pml");
    let doc = Document::from_file(path);
    let root_path = path.parent().unwrap().to_path_buf();
    let mut system = System::new();
    system.add_subsystem(Box::new(pyramid_template::TemplateSubSystem::new(root_path.clone())));
    system.add_subsystem(Box::new(pyramid_animation::AnimationSubSystem::new()));
    system.add_subsystem(Box::new(pyramid_viewport::ViewportSubSystem::new(root_path.clone())));
    system.set_document(doc);

    println!("Starting main loop");
    while system.running {
        system.update();
    }
}
