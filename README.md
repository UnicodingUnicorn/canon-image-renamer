# canon-image-renamer

Recently, my CF card threw a bit of a tantrum. Fortunately, I was about able to recover my photos with the aid of the wonderful [Testdisk](https://www.cgsecurity.org/wiki/TestDisk) utility, but in the process the original filenames were lost. So, I created this utility to help homogenise the recovered files with the rest of my archives.

From what I gather of the Canon (5D Mark II) naming system, images are named sequentially and stuffed into folders of the format "YYYY_MM_DD". Date data is easy enough to pull from the individual images' EXIF data, but the starting index has to be supplied by the user, probably from a previous batch of (non-messed-up) photos.

Finally, I included a system to match two files with different extensions to the same filename, i.e. RAW + JPG, by assuming that if both photos' timestamps match down to the milliseconds they are the same picture.

## Usage

The start index is inclusive, so pass in the number of your last proper image plus one.

```
canon-image-renamer

USAGE:
    canon-image-renamer.exe [OPTIONS] <INPUT_DIRECTORY>

ARGS:
    <INPUT_DIRECTORY>

OPTIONS:
    -h, --help               Print help information
    -o, --output <OUTPUT>    Output directory [default: ./output]
    -s, --start <START>      Starting index for the filenames [default: 1]
```

## Building

It is definitely overkill but I built this project in Rust, so it's just a simple matter of doing `cargo build`. 
