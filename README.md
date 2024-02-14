# obliviate

The goal of this utility is _quickly_ overwrite a file/partition/disk with zeroes to wipe them. _It is not intended for security-sensitive applications!_ Furthermore, it comes with _no guarantees whatsoever_: don't blame me if you (accidentally) wipe out something you cared about or if someone managed to recover data you expected to be unrecoverable.

The utility is quite rough around the edges (error handling is pretty limited, if anything bad happens it will exit with a non-zero status), yet it seems to get the job done. It uses one thread per device in an attempt to maximize I/O bandwidth.

I've only tested this tool on Linux, but in theory it should work on other POSIX-compliant systems as well (albeit with minor changes to the source code)

## Usage

1. Install the [Rust](https://www.rust-lang.org/tools/install) programming language.
2. Build the tool:

```sh
$ cargo build --release
```

3. Run the tool

Note that there aren't any warnings or hand-holding; _obliviate_ will try to wipe whatever you supply as arguments. Note that you'll likely need superuser privileges to do so.

For example:

```sh
$ cargo run --release -- /dev/sde /dev/sdh /dev/sdg ...
```

It'll keep printing a status update like the following until all drives are wiped:

```
0: /dev/sde, 262.5GiB / 2.7TiB, 155.6MiB/sec, 9.39% completed
1: /dev/sdh, 257.9GiB / 2.7TiB, 140.5MiB/sec, 9.23% completed
2: /dev/sdg, 307.5GiB / 2.7TiB, 180.5MiB/sec, 11.01% completed
3: /dev/sdb, 322.7GiB / 3.6TiB, 185.4MiB/sec, 8.66% completed
4: /dev/sdy, 324.3GiB / 2.7TiB, 186.8MiB/sec, 11.61% completed
5: /dev/sdd, 308.3GiB / 3.6TiB, 171.5MiB/sec, 8.28% completed
6: /dev/sdf, 260.7GiB / 2.7TiB, 145.4MiB/sec, 9.33% completed
7: /dev/sdl, 301.8GiB / 3.6TiB, 155.7MiB/sec, 8.10% completed
8: /dev/sda, 272.4GiB / 2.7TiB, 144.5MiB/sec, 9.75% completed
9: /dev/sdp, 326.4GiB / 3.6TiB, 187.7MiB/sec, 8.76% completed
10: /dev/sdc, 276.2GiB / 2.7TiB, 144.1MiB/sec, 9.89% completed
11: /dev/sdi, 261.7GiB / 2.7TiB, 144.1MiB/sec, 9.37% completed
12: /dev/sdx, 372.8GiB / 2.7TiB, 214.7MiB/sec, 13.34% completed
13: /dev/sdr, 334.4GiB / 2.7TiB, 178.2MiB/sec, 11.97% completed
14: /dev/sdm, 349.6GiB / 2.7TiB, 195.8MiB/sec, 12.51% completed
15: /dev/sdt, 352.2GiB / 2.7TiB, 192.0MiB/sec, 12.61% completed
16: /dev/sds, 366.6GiB / 2.7TiB, 189.4MiB/sec, 13.12% completed
17: /dev/sdk, 281.1GiB / 2.7TiB, 156.6MiB/sec, 10.06% completed
18: /dev/sdu, 267.6GiB / 2.7TiB, 149.6MiB/sec, 9.58% completed
19: /dev/sdq, 265.6GiB / 2.7TiB, 150.9MiB/sec, 9.51% completed
20: /dev/sdo, 264.2GiB / 2.7TiB, 136.3MiB/sec, 9.46% completed
21: /dev/sdn, 272.8GiB / 2.7TiB, 151.4MiB/sec, 9.76% completed
22: /dev/sdj, 275.3GiB / 2.7TiB, 154.8MiB/sec, 9.85% completed
23: /dev/sdv, 343.7GiB / 2.7TiB, 194.0MiB/sec, 12.30% completed
```

## Potential improvements

I accept pull requests!

- Remove expectation that the file/device to be an exact multiple of the block size (a file may be slightly expanded if this is not the case)
- Perhaps a warning message after the devices are determined would be nice
- Determine and print how much time is left
- Do not terminate on a failure, rather handle this gracefully and keep going

# License

[zlib](LICENSE)
