use std::rc::Rc;


#[derive(Debug)]
pub enum FsError {
    InvalidPath,
    InvalidDrive,
    MissingDrive,
    AlreadyExists,
    Unsupported,
    NotFound,
    IsNotDirectory,
    IsNotFile,
}
pub type FsResult<T> = Result<T, FsError>;

#[inline]
const fn private_to<'a, T>(private: *const u8) -> &'a T {
    unsafe {
    &*(private.cast())
    }
}

#[inline]
fn private_to_mut<'a, T>(private: *mut u8) -> &'a mut T {
    unsafe {
    &mut *private.cast()
    }
}

#[derive(Clone)]
pub struct Directory<'a> {
    fsops: &'a FsOps<'a>,
    private: *mut u8
}
impl Drop for Directory<'_> {
    #[inline]
    fn drop(&mut self) {
        if let Some(close_dir) = self.fsops.close_dir {
            (close_dir)(self)
        }
    }
}
impl <'a> Directory <'a> {
    pub const fn new(fsops: &'a FsOps<'a>, private: *mut u8) -> Self {
        Self { fsops, private }
    }
    #[inline]
    pub fn find(&self, entry: &str) -> FsResult<DirEntry<'a>> {
        (self.fsops.find.ok_or(FsError::Unsupported)?)(self, entry)
    }

    #[inline]
    pub fn create(&mut self, entry: &str) -> FsResult<()> {
        (self.fsops.create.ok_or(FsError::Unsupported)?)(self, entry)
    }

    #[inline]
    pub fn mkdir(&mut self, entry: &str) -> FsResult<()> {
        (self.fsops.mkdir.ok_or(FsError::Unsupported)?)(self, entry)
    }

    #[inline]
    pub(crate) const fn private_to<T>(&self) -> &T {
        private_to(self.private)
    }

    #[inline]
    pub(crate) fn private_to_mut<T>(&mut self) -> &mut T {
        private_to_mut(self.private)
    }
}

#[derive(Clone)]
pub struct File<'a> {
    fsops: &'a FsOps<'a>,
    private: *mut u8
}
impl Drop for File<'_> {
    #[inline]
    fn drop(&mut self) {
        if let Some(close) = self.fsops.close {
            (close)(self)
        }
    }
}
impl <'a> File <'a> {
    pub const fn new(fsops: &'a FsOps<'a>, private: *mut u8) -> Self {
        Self { fsops, private }
    }
    #[inline]
    pub fn write(&mut self, data: &[u8]) -> FsResult<usize> {
        (self.fsops.write.ok_or(FsError::Unsupported)?)(self, data)
    }
    #[inline]
    pub fn read(&mut self, data: &mut [u8]) -> FsResult<usize> {
        (self.fsops.read.ok_or(FsError::Unsupported)?)(self, data)
    }

    #[inline]
    pub(crate) const fn private_to<T>(&self) -> &T {
        private_to(self.private)
    }

    #[inline]
    pub(crate) fn private_to_mut<T>(&mut self) -> &mut T {
        private_to_mut(self.private)
    }
}


#[derive(Clone)]
pub struct DirEntry<'a> {
    fsops: &'a FsOps<'a>,
    private: *mut u8,
}
impl <'a> DirEntry <'a> {
    pub const fn new(fsops: &'a FsOps<'a>, private: *mut u8) -> Self {
        Self { fsops, private }
    }
    #[inline]
    pub fn open_dir(&self) -> FsResult<Directory<'a>> {
        (self.fsops.open_dir.ok_or(FsError::Unsupported)?)(self)
    }
    #[inline]
    pub fn open(&self) -> FsResult<File<'a>> {
        (self.fsops.open.ok_or(FsError::Unsupported)?)(self)
    }

    #[inline]
    pub(crate) const fn private_to<T>(&self) -> &T {
        private_to(self.private)
    }

    #[inline]
    pub(crate) fn private_to_mut<T>(&mut self) -> &mut T {
        private_to_mut(self.private)
    }
}
impl Drop for DirEntry<'_> {
    #[inline]
    fn drop(&mut self) {
        if let Some(cleanup_entry) = self.fsops.cleanup_entry {
            (cleanup_entry)(self)
        }
    }
}


#[derive(Default)]
pub struct FsOps<'a> {
    pub open_dir     : Option<fn (entry: &DirEntry<'a>) -> FsResult<Directory<'a>>>,
    pub create       : Option<fn (dir: &mut Directory<'a>, name: &str) -> FsResult<()>>,
    pub mkdir        : Option<fn (dir: &mut Directory<'a>, name: &str) -> FsResult<()>>,
    pub find         : Option<fn (dir: &Directory<'a>, name: &str) -> FsResult<DirEntry<'a>>>,
    pub close_dir    : Option<fn (dir: &mut Directory<'a>)>,
    pub cleanup_entry: Option<fn (entry: &mut DirEntry<'a>)>,
    pub unmount      : Option<fn (drive: &mut Drive<'a>)>,
    pub open         : Option<fn (entry: &DirEntry<'a>) -> FsResult<File<'a>>>,
    pub close        : Option<fn (file: &mut File<'a>)>,
    pub write        : Option<fn (file: &mut File<'a>, data: &[u8]) -> FsResult<usize>>,
    pub read         : Option<fn (file: &mut File<'a>, data: &mut [u8]) -> FsResult<usize>>
}
pub struct Drive<'a> {
    pub root: Rc<DirEntry<'a>>, 
    fsops: FsOps<'a>,
    private: *mut u8
}
impl Drop for Drive<'_> {
    fn drop(&mut self) {
        if let Some(unmount) = self.fsops.unmount {
            (unmount)(self)
        }
    }
}
impl <'a> Drive <'a> {
    pub const fn new(root: Rc<DirEntry<'a>>, fsops: FsOps<'a>, private: *mut u8) -> Self {
        Self { fsops, private, root }
    }
}
