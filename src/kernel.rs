use std::rc::Rc;

use crate::vfs::*;
pub struct Kernel<'a> {
    drives: [Option<Drive<'a>>; (b'Z'-b'A') as usize + 1]
}
impl <'a> Kernel <'a> {
    pub const fn new() -> Self {
        Self {
            drives: [const { None }; (b'Z'-b'A'+1) as usize],
        }
    }
    pub fn vfs_find(&mut self, path: &str) -> FsResult<Rc<DirEntry<'a>>> {
        let (drive, mut path) = path.split_once(':').ok_or(FsError::InvalidPath)?;
        if drive.len() != 1 {
            return Err(FsError::InvalidDrive);
        }
        let c = drive.chars().next().unwrap(); 
        if c < 'A' || c > 'Z' { return Err(FsError::InvalidDrive); }
        let dindex = (c as u8 - b'A') as usize;
        let drive = self.drives[dindex].as_ref().ok_or(FsError::MissingDrive)?;
        path = path.strip_prefix('/').ok_or(FsError::InvalidPath)?;
        if path.is_empty() { return Ok(drive.root.clone()) };
        path = path.strip_suffix('/').unwrap_or(path);
        let mut dir = drive.root.open_dir()?;
        while let Some((entry, b)) = path.split_once('/') {
           path = b; 
           let ent = dir.find(entry)?;
           dir = ent.open_dir()?;
        }
        Ok(Rc::new(dir.find(path)?))
    }
    pub fn vfs_create(&mut self, path: &str) -> FsResult<()> {
        let (a, b) = vfs_path_split_parent(path)?;
        let mut parent = self.vfs_find(a)?.open_dir()?;
        parent.create(b)
    }

    pub fn vfs_mkdir(&mut self, path: &str) -> FsResult<()> {
        let (a, b) = vfs_path_split_parent(path)?;
        let mut parent = self.vfs_find(a)?.open_dir()?;
        parent.mkdir(b)
    }
    pub fn vfs_open(&mut self, path: &str) -> FsResult<File<'a>> {
        self.vfs_find(path)?.open()
    }
    pub fn mount_drive(&mut self, letter: char, drive: Drive<'a>) -> FsResult<()> {
        if letter < 'A' || letter > 'Z' { return Err(FsError::InvalidDrive); }
        let dindex = (letter as u8 - b'A') as usize;
        if self.drives[dindex].as_ref().is_some() { return Err(FsError::AlreadyExists); }
        self.drives[dindex] = Some(drive);
        Ok(())
    }
}
fn vfs_path_split_parent(path: &str) -> FsResult<(&str, &str)> {
    let a = path.rfind('/').ok_or(FsError::InvalidPath)?;
    let (p1, p2) = path.split_at(a+1);
    if p2.is_empty() {
        return Err(FsError::InvalidPath);
    }
    Ok((p1,p2))
}
