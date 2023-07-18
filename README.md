# Postim: a toy graphics scripting language

This is a playground project I tinkered together in order to learn Rust.
Do not expect it to be useful.

## Try it out

Create an image file called `test.ppm`. It must be in [PPM format][ppm];
[GnuIMP][gimp] can export that.

[ppm]: https://en.wikipedia.org/wiki/Netpbm_format
[gimp]: https://www.gimp.org/

Then type:

```
cargo run -- -c scripts/rotate.imp test.ppm 0.2 -o out.ppm
```

This rotates the image 0.2 radians (11 degrees), and writes it to `out.ppm`.

## The language

Postim is a stack-based language like [Forth][] or [PostScript][ps].
If you're familiar with "reverse Polish notation", go with that.

[ps]: https://en.wikipedia.org/wiki/PostScript
[forth]: https://en.wikipedia.org/wiki/Forth_(programming_language)


