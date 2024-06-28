use monolith::arena::Arena;
use monolith::intern::StrInterner;

fn main() {
    let _arena = Arena::new(1024);
    let _intern = StrInterner::new(1024);
}
