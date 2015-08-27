extern crate pyramid;
extern crate pyramid_animation;
extern crate pyramid_viewport;

use std::path::Path;

use pyramid::document::*;
use pyramid::system::*;

fn main() {

    let path = Path::new("../examples/test.pml");
    let doc = Document::from_file(path);
    let mut system = System::new();
    system.add_subsystem(Box::new(pyramid_animation::AnimationSubSystem::new()));
    system.add_subsystem(Box::new(pyramid_viewport::ViewportSubSystem::new(path.parent().unwrap().to_path_buf())));
    system.set_document(doc);

    println!("Starting main loop");
    while system.running {
        system.update();
    }
}
