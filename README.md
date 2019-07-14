# rust-wasi-tutorial
Just some simple program with very basic I/O serving as a Rust version of
the excellent [WASI tutorial](https://github.com/CraneStation/wasmtime/blob/master/docs/WASI-tutorial.md).

## Building
Firstly, make sure you are running the latest version of Rust `stable`, `v1.36.0`.
If not, go ahead and install it. BTW, I strongly recommend `rustup` for all
your Rust versioning worries.

Next, install the required target
```
$ rustup target add wasm32-wasi
```

Afterwards, you should be able to cross-compile to WASI by simply running
```
$ cargo build --target=wasm32-wasi
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
RUST_LOG=wasi_common=trace wasmtime -d --dir=. target/wasm32-wasi/debug/main.wasm -- in.txt out.txt
```

As a result, you should get output similar to the following
```
 TRACE wasi_common::hostcalls::fs > fd_prestat_get(fd=3, prestat_ptr=0xffff0)
 TRACE wasi_common::hostcalls     >     -> errno=__WASI_ESUCCESS
 TRACE wasi_common::hostcalls::fs > fd_prestat_dir_name(fd=3, path_ptr=0x110088, path_len=1)
 TRACE wasi_common::hostcalls::fs >      | (path_ptr,path_len)="."
 TRACE wasi_common::hostcalls     >     -> errno=__WASI_ESUCCESS
 TRACE wasi_common::hostcalls::fs > fd_fdstat_get(fd=3, fdstat_ptr=0xfffd8)
 TRACE wasi_common::hostcalls::fs >      | *buf=__wasi_fdstat_t { fs_filetype: 3, fs_flags: 0, fs_rights_base: 264240792, fs_rights_inheriting: 268435455 }
 TRACE wasi_common::hostcalls     >     -> errno=__WASI_ESUCCESS
 TRACE wasi_common::hostcalls::fs > fd_prestat_get(fd=4, prestat_ptr=0xffff0)
 TRACE wasi_common::hostcalls     >     -> errno=__WASI_EBADF
 TRACE wasi_common::hostcalls::misc > environ_sizes_get(environ_count_ptr=0xffff0, environ_size_ptr=0xffffc)
 TRACE wasi_common::hostcalls::misc >      | *environ_count_ptr=0
 TRACE wasi_common::hostcalls::misc >      | *environ_size_ptr=0
 TRACE wasi_common::hostcalls       >     -> errno=__WASI_ESUCCESS
 TRACE wasi_common::hostcalls::misc > args_sizes_get(argc_ptr=0xffffc, argv_buf_size_ptr=0xffff0)
 TRACE wasi_common::hostcalls::misc >      | *argc_ptr=3
 TRACE wasi_common::hostcalls::misc >      | *argv_buf_size_ptr=17
 TRACE wasi_common::hostcalls       >     -> errno=__WASI_ESUCCESS
 TRACE wasi_common::hostcalls::misc > args_get(argv_ptr=0x110088, argv_buf=0x1100a8)
 TRACE wasi_common::hostcalls       >     -> errno=__WASI_ESUCCESS
 TRACE wasi_common::hostcalls::misc > args_sizes_get(argc_ptr=0xffed8, argv_buf_size_ptr=0xffedc)
 TRACE wasi_common::hostcalls::misc >      | *argc_ptr=3
 TRACE wasi_common::hostcalls::misc >      | *argv_buf_size_ptr=17
 TRACE wasi_common::hostcalls       >     -> errno=__WASI_ESUCCESS
 TRACE wasi_common::hostcalls::misc > args_get(argv_ptr=0x110118, argv_buf=0x110128)
 TRACE wasi_common::hostcalls       >     -> errno=__WASI_ESUCCESS
 TRACE wasi_common::hostcalls::fs   > path_open(dirfd=3, dirflags=1, path_ptr=0x110198, path_len=2, oflags=0x0, fs_rights_base=0xf87febe, fs_rights_inheriting=0xf87febe, fs_flags=0x0, fd_out_ptr=0xffcbc)
 TRACE wasi_common::hostcalls::fs   >      | (path_ptr,path_len)="in"
 TRACE wasi_common::hostcalls::fs   >      | *fd=4
 TRACE wasi_common::hostcalls       >     -> errno=__WASI_ESUCCESS
 TRACE wasi_common::hostcalls::fs   > fd_read(fd=4, iovs_ptr=0xffc88, iovs_len=1, nread=0xffc7c)
 TRACE wasi_common::hostcalls::fs   >      | *nread=12
 TRACE wasi_common::hostcalls       >     -> errno=__WASI_ESUCCESS
 TRACE wasi_common::hostcalls::fs   > fd_read(fd=4, iovs_ptr=0xffc88, iovs_len=1, nread=0xffc7c)
 TRACE wasi_common::hostcalls::fs   >      | *nread=0
 TRACE wasi_common::hostcalls       >     -> errno=__WASI_ESUCCESS
 TRACE wasi_common::hostcalls::fs   > path_open(dirfd=3, dirflags=1, path_ptr=0x1101c0, path_len=3, oflags=0x9, fs_rights_base=0xfc7bffd, fs_rights_inheriting=0xfc7bffd, fs_flags=0x0, fd_out_ptr=0xffcbc)
 TRACE wasi_common::hostcalls::fs   >      | (path_ptr,path_len)="out"
 TRACE wasi_common::hostcalls::fs   >      | *fd=4
 TRACE wasi_common::hostcalls       >     -> errno=__WASI_ESUCCESS
 TRACE wasi_common::hostcalls::fs   > fd_write(fd=4, iovs_ptr=0xffce8, iovs_len=1, nwritten=0xffcdc)
 TRACE wasi_common::hostcalls::fs   >      | *nwritten=12
 TRACE wasi_common::hostcalls       >     -> errno=__WASI_ESUCCESS
 TRACE wasi_common::hostcalls::fs   > fd_close(fd=4)
 TRACE wasi_common::hostcalls       >     -> errno=__WASI_ESUCCESS
 TRACE wasi_common::hostcalls::fs   > fd_close(fd=4)
 TRACE wasi_common::hostcalls       >     -> errno=__WASI_EBADF
```

Pretty neat, isn't it? ;-)

## License
[The Unlicense](LICENSE)
