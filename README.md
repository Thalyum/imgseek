# imgseek

Tool to find and search a list of binaries inside a flash storage dump image. In
fact, this can be used to search any binary into another binary, but was
designed to search based on block alignment.

This could be improved later.

Given a flash image, and a list of binaries, it will output a schema of the
flash layout.

Performance analysis not yet available.

## Installation

You can use `Cargo` to build the tool. Just run:
```sh
Cargo build --release
```
To build the release version. Then install it to you preferred location:
```sh
cp target/release/imgseek ~/.cargo/bin
```

## Usage

Here is the extract of the help message
```
imgseek 0.1.1
Paul-Erwan RIO <paulerwan.rio@proton.me>
Seeker tool for binaries in flash images

USAGE:
    imgseek [OPTIONS] --binaries <binaries_list>... --image <flash_image>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -b, --binaries <binaries_list>...    List of binaries to search for
    -s, --size <bsize>                   Page / block size [default: 512]
    -i, --image <flash_image>            The flash image to search in
        --h_scale <h_scale>              Horizontal scaling, default is half of the term size
        --v_scale <v_scale>              Vertical scaling, default is half of the term size
```

Example:
```sh
imgseek --image total_image --binaries image1 image2
```

Here is a quick example. First create a dummy flash image
```sh
dd if=/dev/random of=total_image bs=512 count=8
```
Then create two dummy binaries:
```sh
dd if=total_image of=image1 bs=512 skip=1 count=1
dd if=total_image of=image2 bs=512 skip=5 count=1
```
Then run:
```sh
imgseek --image total_image --binaries image1 image2
```

Output:
```
┌────────────────────────────────┐ <-- 0x00000000
│                                │
│                                │
│                                │
├────────────────────────────────┤ <-- 0x00000200
│00000000000000000000000000000000│
│00000000000000000000000000000000│
│00000000000000000000000000000000│
├────────────────────────────────┤ <-- 0x00000400
│                                │
│                                │
│                                │
├────────────────────────────────┤ <-- 0x00000a00
│11111111111111111111111111111111│
│11111111111111111111111111111111│
│11111111111111111111111111111111│
├────────────────────────────────┤ <-- 0x00000c00
│                                │
│                                │
│                                │
└────────────────────────────────┘ <-- 0x00001000
0: 'image1'
1: 'image2'
```

If the binaries overlap, as with this example:
```sh
dd if=total_image of=image1 bs=512 skip=1 count=3
dd if=total_image of=image2 bs=512 skip=2 count=3
```

The the result will be displayed using multiple columns:
```
┌───────────────────────────────┐ <-- 0x00000000
│                               │
│                               │
│                               │
├───────────────┐               │ <-- 0x00000200
│000000000000000│               │
│000000000000000│               │
│000000000000000│               │
│000000000000000├───────────────┤ <-- 0x00000400
│000000000000000│111111111111111│
│000000000000000│111111111111111│
│000000000000000│111111111111111│
├───────────────┤111111111111111│ <-- 0x00000800
│               │111111111111111│
│               │111111111111111│
│               │111111111111111│
│               └───────────────┤ <-- 0x00000a00
│                               │
│                               │
│                               │
└───────────────────────────────┘ <-- 0x00001000
0: 'image1'
1: 'image2'
```


## License

Under MIT License
