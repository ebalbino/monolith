use monolith::arena::Arena;
use monolith::platform::unix::filesystem::Filesystem;

fn main() {
    let arena = Arena::new(1024 * 1024 * 16);
    let filesystem = Filesystem::new();

    let file = filesystem.open("./data/shaders/basic.vert");
    let data = file.read(&arena);
    let string = core::str::from_utf8(&data).unwrap();

    println!("{} bytes", file.size());
    println!("{}", string);

    file.append("#define TEST 0\n".as_bytes())
}
