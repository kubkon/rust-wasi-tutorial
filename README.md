# rust-wasi-tutorial
Just some simple program with very basic I/O serving as a Rust version of
the excellent [WASI tutorial](https://github.com/CraneStation/wasmtime/blob/master/docs/WASI-tutorial.md).

## Building
Firstly, make sure you are running the latest version of Rust `nightly`.
If not, go ahead and install it. BTW, I strongly recommend `rustup` for all
your Rust versioning worries.

Next, install the required target
```
$ rustup target add wasm32-wasi --toolchain nightly
```

Afterwards, you should be able to cross-compile to WASI by simply running
```
$ cargo +nightly build --target=wasm32-wasi
```

## Running
Just for fun, let's test it out using [wasmtime](https://github.com/CraneStation/wasmtime)
WASI runtime.

Create sample input
```
echo 'WASI is a lot of fun!' > in.txt
```

Run it using `wasmtime`
```
$ wasmtime --dir=. target/wasm32-wasi/debug/main.wasm in.txt out.txt
```

As a result, you should have `out.txt` file created with the same contents as `in.txt` :-)

### Bonus: exploring the syscalls
For the curious ones, here's a simple way to trace back the syscalls used by our WASI program
in `wasmtime`
```
env RUST_LOG=wasmtime_wasi=trace wasmtime -d --dir=. target/wasm32-wasi/debug/main.wasm in.txt out.txt
```

As a result, you should get output similar to the following
```
 TRACE wasmtime_wasi::syscalls > fd_prestat_get(fd=3, buf=0xffff0)
 TRACE wasmtime_wasi::syscalls >     -> errno=__WASI_ESUCCESS
 TRACE wasmtime_wasi::syscalls > fd_prestat_dir_name(fd=3, path=0x110088, path_len=1)
 TRACE wasmtime_wasi::syscalls >      | (path,path_len)=Ok("\u{0}")
 TRACE wasmtime_wasi::syscalls >     -> errno=__WASI_ESUCCESS
 TRACE wasmtime_wasi::syscalls > fd_fdstat_get(fd=3, buf=0xfffd8)
 TRACE wasmtime_wasi::syscalls >      | *buf=__wasi_fdstat_t { fs_filetype: 3, fs_flags: 0, fs_rights_base: 264240792, fs_rights_inheriting: 268435455 }
 TRACE wasmtime_wasi::syscalls >     -> errno=__WASI_ESUCCESS
 TRACE wasmtime_wasi::syscalls > fd_prestat_get(fd=4, buf=0xffff0)
 TRACE wasmtime_wasi::syscalls >     -> errno=__WASI_EBADF
 TRACE wasmtime_wasi::syscalls > environ_sizes_get(environ_count=0xffff0, environ_buf_size=0xffffc)
 TRACE wasmtime_wasi::syscalls >      | *environ_count=0
 TRACE wasmtime_wasi::syscalls >      | *environ_buf_size=0
 TRACE wasmtime_wasi::syscalls >     -> errno=__WASI_ESUCCESS
 TRACE wasmtime_wasi::syscalls > args_sizes_get(argc=0xffffc, argv_buf_size=0xffff0)
 TRACE wasmtime_wasi::syscalls >      | *argc=3
 TRACE wasmtime_wasi::syscalls >      | *argv_buf_size=25
 TRACE wasmtime_wasi::syscalls >     -> errno=__WASI_ESUCCESS
 TRACE wasmtime_wasi::syscalls > args_get(argv=0x110088, argv_buf=0x1100a8)
 TRACE wasmtime_wasi::syscalls >     -> errno=__WASI_ESUCCESS
 TRACE wasmtime_wasi::syscalls > args_sizes_get(argc=0xffed8, argv_buf_size=0xffedc)
 TRACE wasmtime_wasi::syscalls >      | *argc=3
 TRACE wasmtime_wasi::syscalls >      | *argv_buf_size=25
 TRACE wasmtime_wasi::syscalls >     -> errno=__WASI_ESUCCESS
 TRACE wasmtime_wasi::syscalls > args_get(argv=0x110120, argv_buf=0x110130)
 TRACE wasmtime_wasi::syscalls >     -> errno=__WASI_ESUCCESS
 TRACE wasmtime_wasi::syscalls > path_open(dirfd=3, dirflags=1, path=0x1101d0, path_len=6, oflags=0x0, fs_rights_base=0xf87febe, fs_rights_inheriting=0xf87febe, fs_flags=0x0, fd=0xffe4c)
 TRACE wasmtime_wasi::syscalls >      | (path,path_len)=Ok("in.txt")
 TRACE wasmtime_wasi::syscalls >      | *fd=5
 TRACE wasmtime_wasi::syscalls >     -> errno=__WASI_ESUCCESS
 TRACE wasmtime_wasi::syscalls > fd_read(fd=5, iovs=0xffe18, iovs_len=1, nread=0xffe0c)
 TRACE wasmtime_wasi::syscalls >      | *nread=15
 TRACE wasmtime_wasi::syscalls >     -> errno=__WASI_ESUCCESS
 TRACE wasmtime_wasi::syscalls > fd_read(fd=5, iovs=0xffe18, iovs_len=1, nread=0xffe0c)
 TRACE wasmtime_wasi::syscalls >      | *nread=0
 TRACE wasmtime_wasi::syscalls >     -> errno=__WASI_ESUCCESS
 TRACE wasmtime_wasi::syscalls > path_open(dirfd=3, dirflags=1, path=0x1101f8, path_len=7, oflags=0x9, fs_rights_base=0xfc7bffd, fs_rights_inheriting=0xfc7bffd, fs_flags=0x0, fd=0xffe4c)
 TRACE wasmtime_wasi::syscalls >      | (path,path_len)=Ok("out.txt")
 TRACE wasmtime_wasi::syscalls >      | *fd=12
 TRACE wasmtime_wasi::syscalls >     -> errno=__WASI_ESUCCESS
 TRACE wasmtime_wasi::syscalls > fd_write(fd=12, iovs=0xffe78, iovs_len=1, nwritten=0xffe6c)
 TRACE wasmtime_wasi::syscalls >      | *nwritten=15
 TRACE wasmtime_wasi::syscalls >     -> errno=__WASI_ESUCCESS
 TRACE wasmtime_wasi::syscalls > fd_close(fd=12)
 TRACE wasmtime_wasi::syscalls >     -> errno=__WASI_ESUCCESS
 TRACE wasmtime_wasi::syscalls > fd_close(fd=5)
 TRACE wasmtime_wasi::syscalls >     -> errno=__WASI_ESUCCESS
```

Pretty neat, isn't it? ;-)

## License
[The Unlicense](LICENSE)
