use std::rc::Rc;
use crate::vfs::*;

struct TmpfsDirectory<'a> {
    entries: Vec<DirEntry<'a>>,
}
enum TmpfsDirEntryKind<'a> {
    Directory { dir: Directory<'a> },
    File { file: File<'a> },
}
struct TmpfsDirEntry<'a> {
    name: String,
    kind: TmpfsDirEntryKind<'a>,
}
struct TmpfsFile {
    data: Vec<u8>
}


fn tmpfs_open_dir<'a>(entry: &DirEntry<'a>) -> FsResult<Directory<'static>> {
    let entry = entry.private_to::<TmpfsDirEntry>();
    match &entry.kind {
        TmpfsDirEntryKind::Directory { dir } => {
             Ok(dir.clone())
        }
        _ => Err(FsError::IsNotDirectory),
    }
}
fn tmpfs_create<'a>(dir: &mut Directory<'a>, name: &str) -> FsResult<()> {
    dir.private_to_mut::<TmpfsDirectory>().entries.push(DirEntry::new(&TMPFS_FSOPS, Box::leak(Box::new(TmpfsDirEntry { name: name.to_string(), kind: TmpfsDirEntryKind::File { file: File::new(&TMPFS_FSOPS, Box::leak(Box::new(TmpfsFile { data: Vec::new() })) as *mut _ as *mut u8) } } )) as *mut _ as *mut u8));
    Ok(())
}
fn tmpfs_mkdir<'a>(dir: &mut Directory<'a>, name: &str) -> FsResult<()> {
    dir.private_to_mut::<TmpfsDirectory>().entries.push(DirEntry::new(&TMPFS_FSOPS, Box::leak(Box::new(TmpfsDirEntry { name: name.to_string(), kind: TmpfsDirEntryKind::Directory { dir: Directory::new(&TMPFS_FSOPS, Box::leak(Box::new(TmpfsDirectory { entries: Vec::new() })) as *mut _ as *mut u8) } })) as *mut _ as *mut u8));
    Ok(())
}
fn tmpfs_find<'a>(dir: &Directory<'a>, name: &str) -> FsResult<DirEntry<'static>> {
    let dir = dir.private_to::<TmpfsDirectory>();
    if let Some(v) = dir.entries.iter().find(|x| x.private_to::<TmpfsDirEntry>().name == name) {
        return Ok(v.clone())
    }
    Err(FsError::NotFound)
}

fn tmpfs_open<'a>(entry: &DirEntry<'a>) -> FsResult<File<'a>> {
    let entry = entry.private_to::<TmpfsDirEntry>();
    match &entry.kind {
        TmpfsDirEntryKind::File { file } => {
            Ok(file.clone())
        }
        _ => Err(FsError::IsNotFile),
    }
}

fn tmpfs_write(f: &mut File, data: &[u8]) -> FsResult<usize> {
    let f = f.private_to_mut::<TmpfsFile>();
    f.data.extend(data);
    Ok(data.len())
}

fn tmpfs_read(f: &mut File, data: &mut [u8]) -> FsResult<usize> {
    let f = f.private_to_mut::<TmpfsFile>();

    let n = data.len().min(f.data.len());
    data[..n].copy_from_slice(&f.data[..n]);
    Ok(n)
}

const TMPFS_FSOPS: FsOps<'static> = FsOps {
    open_dir     : Some(tmpfs_open_dir), 
    create       : Some(tmpfs_create),
    mkdir        : Some(tmpfs_mkdir),
    find         : Some(tmpfs_find), 
    close_dir    : None, 
    cleanup_entry: None, 
    unmount      : None, 
    open         : Some(tmpfs_open),
    close        : None,
    write        : Some(tmpfs_write),
    read         : Some(tmpfs_read),
};
#[inline]
fn tmpfs_empty_dir_private() -> *mut u8 {
    Box::leak(
        Box::new(
            TmpfsDirectory {
                entries: Vec::new()
            }
        )
    ) as *mut _ as *mut u8
}

#[inline]
fn tmpfs_empty_dir() -> Directory<'static> {
    Directory::new(
        &TMPFS_FSOPS,
        tmpfs_empty_dir_private()
    )
}

fn tmpfs_root() -> DirEntry<'static> {
    DirEntry::new(
        &TMPFS_FSOPS,
        {
            Box::leak(Box::new(
                TmpfsDirEntry {
                    name: "/".to_string(),
                    kind: TmpfsDirEntryKind::Directory { dir: tmpfs_empty_dir() }
                }
            )) as *mut _ as *mut u8 
        }
    )
}
pub fn tmpfs_drive() -> Drive<'static> {
    Drive::new(Rc::new(tmpfs_root()), TMPFS_FSOPS, 0 as *mut _)
}
