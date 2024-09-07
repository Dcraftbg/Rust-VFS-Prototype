# Rust VFS Prototype

A Windows style (Drive lettering) VFS Prototype written in Rust.

Also includes a really simplistic example implementation of a Tmpfs.

## License

All the code falls under the MIT license

## Purpose 

Simplicity. The entire VFS is made to be simple and easy to adapt to (albeit lacking features like Inodes and Inode safety). Its mainly supposed to show off how basic implementations of a VFS can be made, in a "C like" way, without any overhead that may come with using traits
