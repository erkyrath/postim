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
cargo run -- test.ppm 0.2 scripts/rotate.imp -o out.ppm
```

This rotates the image 0.2 radians (11 degrees), and writes it to `out.ppm`.

If you find your script is slow, do `cargo run --release` to run Postim
in release mode.

## The language

Postim is a stack-based language like [Forth][] or [PostScript][ps].
If you're familiar with "reverse Polish notation", go with that.

[ps]: https://en.wikipedia.org/wiki/PostScript
[forth]: https://en.wikipedia.org/wiki/Forth_(programming_language)

The `rotate.imp` script mentioned above looks like this:

```
# rotate.img IMAGE THETA

# Create a new image which is the original rotated THETA radians.

# Store the two values passed in on the command line.
>>th  >>img

# Compute the sine and cosine of the angle th.
th cos >>thcos
th sin >>thsin

# Compute half the size of the image and store as halfwidth, halfheight.
img size split 0.5 * >>halfheight 0.5 * >>halfwidth

# Now take the image and apply a projection to create a new image.

# The section in braces is a projection function which takes two
# values (on the stack) and returns two new values (on the stack).
# That is, the function maps (x,y) to (x',y'). The project operator
# uses this function to turn img into a new image, which is a rotation
# of the original.

img {
  halfheight - >>yval
  halfwidth - >>xval
  thcos xval * thsin yval * + halfwidth +
  thcos yval * thsin xval * - halfheight +
} project

# The new image is left on the stack.
```

We start with two values on the stack, taken from the command line:
the image `test.ppm` and the number `0.2`. We then run `rotate.imp`.

The `>>foo` notation means "store a value as variable `foo`."
We store the input values as `th` and `img`. (Remember that the angle
`0.2` was pushed second, so it is popped first!)

The `-o out.ppm` command-line argument writes out the image left on the
stack.

You can include any number of scripts, arguments, and operators on the
command line. This comment removes the blue component from an image
and then rotates it 45 degrees:

```
cargo run -- test.ppm '{ split pop 0.0 } map' 0.785 scripts/rotate.imp -o out.ppm
```
