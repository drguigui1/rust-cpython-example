# rs-bind-py
Use rust to extend python (Rust bindings for python)


## Use

Build the lib:

```
cargo build --release
```


Copy the .so file in local directory:

```
cp target/release/libtest_bind_lib.so ./test_bind_lib.so
```


Test the lib in python:

```
python
>>> import test_bind_lib
>>> test_bind_lib.get_result("Heyyy")
```
