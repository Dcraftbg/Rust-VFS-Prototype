#![allow(dead_code)]
use kernel::Kernel;
use tmpfs::tmpfs_drive;
mod vfs;
mod tmpfs;
mod kernel;
fn main() {
    let mut kernel = Kernel::new(); 
    kernel.mount_drive('A', tmpfs_drive()).expect("Failed to mount A drive");
    kernel.vfs_mkdir("A:/foo").unwrap();
    kernel.vfs_create("A:/foo/bar.txt").unwrap();
    {
       let mut f = kernel.vfs_open("A:/foo/bar.txt").unwrap();
       f.write(b"Hello World!").unwrap();
    }
    
    {
       let mut f = kernel.vfs_open("A:/foo/bar.txt").unwrap();
       let mut buf = vec![0;50];
       let n = f.read(&mut buf).unwrap();
       println!("Read {}", std::str::from_utf8(&buf[..n]).unwrap());
    }
    /*
    let root = kernel.vfs_find("A:/").expect("Could not find A:/bar.txt");
    let mut dir = root.open_dir().expect("Failed to open root");
    dir.create("bar.txt").expect("bar.txt");

    let mut f = kernel.vfs_find("A:/bar.txt").expect("bar.txt").open().expect("Failed to open");
    let _ = f.write(b"Hello World").expect("Writing to file");
    let mut buf = vec![0; 50];
    let n = f.read(&mut buf).unwrap();
    println!("Read file data: {}",std::str::from_utf8(&buf[..n]).unwrap());
    */
}
