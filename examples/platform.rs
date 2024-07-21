use monolith::platform::unix::filesystem::{Filesystem, INode};
use std::ops::Deref;

fn print_entries(root_node: &Vec<INode>) {
    for entry in root_node {
        match entry {
            INode::File(string) => {
                println!("{}", string.deref());
            }
            INode::Directory(nodes) => {
                print_entries(&nodes);
            }
        }
    }
}

fn main() {
    let filesystem = Filesystem::new(".");
    let root = filesystem.read();

    print_entries(&root);
}
