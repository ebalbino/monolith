use crate::arena::{Arena, ArenaString, ArenaView};
use crate::intern::StrPool;
use alloc::collections::BTreeMap;
use core::cell::{OnceCell, RefCell};
use core::mem;
use libc::{
    fstat, open, read, stat, S_IFBLK, S_IFCHR, S_IFDIR, S_IFIFO, S_IFLNK, S_IFMT, S_IFREG, S_IFSOCK,
};
use libc::{write, O_CREAT, O_RDWR};

pub struct Filesystem<'a> {
    arena: Arena,
    strings: StrPool,
    loaded: RefCell<BTreeMap<&'a str, i32>>,
}

pub struct File {
    handle: i32,
    stat: OnceCell<stat>,
}

pub enum FileType {
    BlockDevice,
    CharacterDevice,
    Directory,
    Pipe,
    SymLink,
    Regular,
    Socket,
    Unknown,
}

fn cstr(arena: &Arena, text: &str) -> *mut i8 {
    let len = text.len() + 1;
    let mut string = arena.allocate::<i8>(len).unwrap();

    unsafe {
        core::ptr::copy_nonoverlapping(text.as_ptr() as *const i8, string.as_mut_ptr(), text.len());
    }

    string.as_mut_ptr()
}

impl<'a> Filesystem<'a> {
    pub fn new() -> Self {
        let arena = Arena::new(1024 * 1024);

        Filesystem {
            arena,
            strings: StrPool::new(1024 * 10),
            loaded: RefCell::new(BTreeMap::new()),
        }
    }

    pub fn open(&self, path: &'a str) -> File {
        let mut loaded = self.loaded.borrow_mut();
        let path = self.strings.intern(path).unwrap();
        let entry = loaded.get(&path);

        match entry {
            None => {
                let cpath = cstr(&self.arena, &path);
                let handle = unsafe { open(cpath, O_RDWR | O_CREAT) };

                loaded.insert(path, handle);

                File {
                    handle,
                    stat: OnceCell::new(),
                }
            }
            Some(handle) => File {
                handle: *handle,
                stat: OnceCell::new(),
            },
        }
    }
}

impl File {
    pub fn stat(&self) -> stat {
        match self.stat.get() {
            None => {
                let stat = unsafe {
                    let mut data: stat = mem::zeroed();
                    fstat(self.handle, &mut data);
                    data
                };

                let _ = self.stat.set(stat);
                self.stat()
            }
            Some(stat) => *stat,
        }
    }

    pub fn size(&self) -> i64 {
        match self.stat.get() {
            None => {
                self.stat();
                self.size()
            }
            Some(stat) => {
                return stat.st_size;
            }
        }
    }

    pub fn blocks(&self) -> i64 {
        match self.stat.get() {
            None => {
                self.stat();
                self.blocks()
            }
            Some(stat) => {
                return stat.st_blocks;
            }
        }
    }

    pub fn file_type(&self) -> FileType {
        match self.stat.get() {
            None => {
                self.stat();
                self.file_type()
            }
            Some(stat) => match stat.st_mode & S_IFMT {
                S_IFBLK => FileType::BlockDevice,
                S_IFCHR => FileType::CharacterDevice,
                S_IFDIR => FileType::Directory,
                S_IFIFO => FileType::Pipe,
                S_IFLNK => FileType::SymLink,
                S_IFREG => FileType::Regular,
                S_IFSOCK => FileType::Socket,
                _ => FileType::Unknown,
            },
        }
    }

    pub fn read(&self, arena: &Arena) -> ArenaView<u8> {
        let size = self.size();
        let mut buf = arena.allocate::<u8>(size as usize).unwrap();

        unsafe {
            read(
                self.handle,
                buf.as_mut_ptr() as *mut core::ffi::c_void,
                size as usize,
            );
        }

        buf
    }

    pub fn append(&self, data: &[u8]) {
        unsafe {
            write(
                self.handle,
                data.as_ptr() as *const core::ffi::c_void,
                data.len(),
            );
        }
    }
}
