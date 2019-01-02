# Soma: Your one-stop CTF problem management tool

[![Build Status](https://dev.azure.com/plus-postech/soma/_apis/build/status/PLUS-POSTECH.soma?branchName=master)](https://dev.azure.com/plus-postech/soma/_build/latest?definitionId=1?branchName=master)

## What is Soma?

Soma is a CTF problem management tool.

Soma helps to manage and distribute CTF problems after the contests.

### For problem solvers

*This section contains a few commands that are under development.*

Downloading and running the problem is as easy as running three commands.

```
soma add https://github.com/PLUS-POSTECH/simple-bof.git
soma pull simple-bof
soma run simple-bof --port 7000
```

CTF problems often contain public files. You can also fetch them easily with soma.

```
soma fetch simple-bof  # this will copy public files to the current working directory
```

### For problem setters

*This section contains a few commands that are under development.*

Add `soma.toml` to your project root directory that describes your problem.
The config file below shows an example of it.

```toml
name = "simple-bof"

[[executable]]
path = "build/simple-bof"
public = true

[[readonly]]
path = "flag"

[binary]
os = "ubuntu:16.04"
entry = "./simple-bof"
```

That's all! Soma gets enough information to run your binary from these 12 lines of configuration.

Soma will use reasonable default value for the other things not specified such as
default working directory, file permissions, fork daemon, and standard stream buffering.
Of course they can be manually configured if needed :)

## Current Status

Soma is in **pre-alpha** stage. Currently, Soma does not have any stable release, and everything is subject to change.

The initial 0.1.0 release will contain the features listed in the issues #4.
Issues related to 0.1.0 release are marked with `0.1.0` milestone.

Soma team is hoping to ship it in the first quarter of 2019.

### Roadmap

* Implement core commands. (priority: high)
* Write tests. (priority: high)
* Better documentation of features. (priority: medium)
* Support multiple problems in a single repository. (priority: medium)
* Support multiple containers for a single problem. (priority: medium)
* Support cloud deployment such as AWS, GCP, Azure as well as local deployment. (priority: low)


## Development

### Prerequisites

* Install Rust stable toolchain.
* Install `openssl` (Required by `openssl-sys` crate).
* Install `rustfmt`.
    * `rustup component add rustfmt`
* Copy files in `hooks` directory to `.git/hooks`.

### Testing, Building, and Running

Soma is written with Rust and utilizes Cargo as a building and testing system.

You can test, build, and run with the following command.

```
cargo test
cargo build
cargo run
```


## License

Licensed under either of
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.


### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you shall be dual licensed as above, without any additional terms or conditions.
