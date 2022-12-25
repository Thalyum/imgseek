# flash-img-seeker

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
cp target/release/flash-img-seeker ~/.cargo/bin
```

## Usage

Here is the extract of the help message
```
flash-img-seeker 0.1.1
Paul-Erwan RIO <paulerwan.rio@proton.me>
Seeker tool for binaries in flash images

USAGE:
    flash-img-seeker [OPTIONS] --binaries <binaries_list>... --image <flash_image>

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
flash-img-seeker --image total_image --binaries image1 image2
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
flash-img-seeker --image total_image --binaries image1 image2
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

## Performance

Using my old laptop (4 core - Intel i3-4005U CPU @ 1.70GHz), generation of 1GiB
image:
```sh
dd if=/dev/urandom of=flash_dummy bs=1M count=1024
dd if=flash_dummy of=test_image bs=1M count=45 skip=45
dd if=flash_dummy of=test_image2 bs=1M count=145 skip=86
```
Running the tool 30 times:
```
 1,22s user 0,35s system 99% cpu 1,584 total
 1,17s user 0,35s system 99% cpu 1,528 total
 1,05s user 0,36s system 99% cpu 1,424 total
 1,16s user 0,34s system 98% cpu 1,522 total
 1,11s user 0,42s system 99% cpu 1,543 total
 1,17s user 0,44s system 98% cpu 1,637 total
 1,05s user 0,39s system 99% cpu 1,451 total
 1,11s user 0,37s system 99% cpu 1,487 total
 1,11s user 0,43s system 99% cpu 1,549 total
 1,08s user 0,36s system 98% cpu 1,462 total
 1,03s user 0,41s system 99% cpu 1,441 total
 1,07s user 0,39s system 97% cpu 1,491 total
 1,07s user 0,38s system 99% cpu 1,468 total
 1,11s user 0,39s system 98% cpu 1,532 total
 0,97s user 0,41s system 99% cpu 1,397 total
 1,12s user 0,41s system 96% cpu 1,581 total
 1,05s user 0,39s system 99% cpu 1,449 total
 1,09s user 0,38s system 97% cpu 1,515 total
 1,04s user 0,41s system 99% cpu 1,453 total
 1,22s user 0,37s system 97% cpu 1,635 total
 1,04s user 0,44s system 98% cpu 1,501 total
 1,21s user 0,39s system 97% cpu 1,637 total
 1,09s user 0,35s system 98% cpu 1,449 total
 1,14s user 0,32s system 97% cpu 1,505 total
 1,01s user 0,41s system 97% cpu 1,463 total
 1,08s user 0,42s system 95% cpu 1,566 total
 0,98s user 0,35s system 99% cpu 1,333 total
 1,09s user 0,41s system 97% cpu 1,533 total
 1,13s user 0,29s system 96% cpu 1,473 total
 1,13s user 0,37s system 98% cpu 1,531 total
```


## License

Under MIT License
