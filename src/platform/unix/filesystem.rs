use crate::arena::{Arena, ArenaSlice, ArenaString};
use crate::intern::StrPool;
use alloc::collections::BTreeMap;
use alloc::fmt::Write;
use alloc::vec::Vec;
use core::cell::{OnceCell, Ref, RefCell};
use core::mem;
use libc::{
    close, closedir, fstat, open, opendir, read, readdir, stat, DT_DIR, DT_REG, S_IFBLK, S_IFCHR,
    S_IFDIR, S_IFIFO, S_IFLNK, S_IFMT, S_IFREG, S_IFSOCK,
};
use libc::{write, O_CREAT, O_RDWR};

pub struct Filesystem<'a> {
    arena: Arena,
    root: ArenaString,
    strings: StrPool,
    nodes: RefCell<Vec<INode>>,
    loaded: RefCell<BTreeMap<&'a str, i32>>,
}

pub enum INode {
    Directory(Vec<INode>),
    File(ArenaString),
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

fn read_directory(arena: &Arena, path: &str) -> Vec<INode> {
    let mut nodes = Vec::with_capacity(16);

    unsafe {
        let dirp = opendir(cstr(arena, path));
        let mut entry = readdir(dirp);

        while !entry.is_null() {
            let inner = *entry;
            let name_cstr = core::ffi::CStr::from_ptr(inner.d_name.as_ptr());
            let name = name_cstr.to_str().unwrap();

            match name {
                "." | ".." | ".git" => {}
                _ => {
                    let mut file_path = arena.allocate_string(path.len() + name.len() + 1).unwrap();
                    let _ = write!(&mut file_path, "{}/{}", path, name);
                    match inner.d_type {
                        DT_DIR => {
                            let inner_nodes = read_directory(arena, &file_path);
                            nodes.push(INode::Directory(inner_nodes));
                        }
                        DT_REG => {
                            nodes.push(INode::File(arena.push_string(&file_path).unwrap()));
                        }
                        _ => {}
                    }
                }
            }

            entry = readdir(dirp);
        }

        closedir(dirp);
    }

    nodes
}

impl<'a> Filesystem<'a> {
    pub fn new(root: &str) -> Self {
        let arena = Arena::new(1024 * 1024);
        let root = arena.push_string(root).unwrap();

        Filesystem {
            arena,
            root,
            nodes: RefCell::new(Vec::new()),
            strings: StrPool::new(1024 * 10),
            loaded: RefCell::new(BTreeMap::new()),
        }
    }

    pub fn read(&self) -> Ref<Vec<INode>> {
        let nodes = read_directory(&self.arena, &self.root);
        let _ = self.nodes.replace(nodes);
        self.nodes.borrow()
    }

    pub fn load(&self, path: &'a str) -> File {
        let mut loaded = self.loaded.borrow_mut();
        let path = self.strings.intern(path).unwrap();
        let entry = loaded.get(&path);

        match entry {
            None => {
                let cpath = cstr(&self.arena, path);
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

    pub fn unload(&self, path: &'a str) {
        let mut loaded = self.loaded.borrow_mut();
        let path = self.strings.intern(path).unwrap();
        let entry = loaded.get(&path);

        match entry {
            None => {}
            Some(handle) => unsafe {
                close(*handle);
                loaded.remove(&path);
            },
        }
    }

    pub fn loaded(&self) -> Ref<BTreeMap<&'a str, i32>> {
        self.loaded.borrow()
    }
}

impl Drop for Filesystem<'_> {
    fn drop(&mut self) {
        for (_path, desc) in self.loaded.borrow().iter() {
            unsafe {
                close(*desc);
            }
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
            Some(stat) => stat.st_size,
        }
    }

    pub fn blocks(&self) -> i64 {
        match self.stat.get() {
            None => {
                self.stat();
                self.blocks()
            }
            Some(stat) => stat.st_blocks,
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

    pub fn read(&self, arena: &Arena) -> ArenaSlice<u8> {
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

    pub fn read_to_string(&self, arena: &Arena) -> ArenaString {
        let inner = self.read(arena);
        ArenaString::from_view(inner)
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
