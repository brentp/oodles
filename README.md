# oodles 

oodles of small bioinformatics tools in a single binary (using [noodles](https://github.com/zaeleus/noodles))


## base-qual0

set selected base-quality scores in a bam to 0

```
Usage: oodles base-qual0 [OPTIONS] <INPUT> [OUTPUT]

Arguments:
  <INPUT>   input bam file
  [OUTPUT]  output bam file [default: /dev/stdout]

Options:
      --left-read1 <LEFT_READ1>    clip this many bases from left end of read1 (as aligned) [default: 0]
      --right-read1 <RIGHT_READ1>  clip this many bases from right end of read1 (as aligned) [default: 0]
      --left-read2 <LEFT_READ2>    clip this many bases from left end of read2 (as aligned) [default: 0]
      --right-read2 <RIGHT_READ2>  clip this many bases from right end of read2 (as aligned) [default: 0]
      -h, --help                       Print help
```
