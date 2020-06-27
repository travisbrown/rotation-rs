# Document rotation detection

This is my first attempt at compiling Rust to [WebAssembly](https://webassembly.org/).
It's a Saturday project and probably won't go anywhere or be maintained in any way. I know nothing
about WebAssembly and very little about Rust, so it's unlikely to illustrate best practices.

The application is an OCR preprocessor that takes a scanned document (which may be rotated) and
attempts to estimate the angle of rotation. It does this by computing the Fourier transform of the
image using [RustFFT](https://github.com/awelkie/RustFFT).
If this was real code it'd probably be better to use
[Rust's OpenCV bindings](https://github.com/twistedfall/opencv-rust),
but I had to stick to pure Rust libraries since the goal was to run this in the browser via
WebAssembly.

The code includes ideas from [here](https://stackoverflow.com/a/33707537/334519) and
[here](https://blog.stackpath.com/image-manipulation/), as well as the official Rust documentation.

If you have Rust, Cargo, and npm installed and you've run `cargo install wasm-pack` at some point,
you should be able to build the web application like this:

```bash
$ wasm-pack build
$ cd www/
$ npm install
$ npm run start
```

The application should then be available at [localhost:8080](http://localhost:8080/):

![Web application](https://github.com/travisbrown/rotation-rs/blob/main/examples/screenshot-01.png?raw=true)

You can also build a native CLI, which you can test against synthetic data like this
(if you have [ImageMagick](https://imagemagick.org/) installed):

```bash
$ cargo build --release
$ convert -rotate 5 examples/lorem.png lorem-5.png
$ target/release/rotation lorem-5.png
5.0040045
$ convert -rotate 7.5 examples/lorem.png lorem-7.5.png
$ target/release/rotation lorem-7.5.png
7.487998
$ convert -rotate -1.8 examples/lorem.png lorem-1.8.png
$ target/release/rotation lorem-1.8.png
-1.746
```

It's not great, but it's not terrible for a few dozen lines of code.

The CLI can also save the FFT spectrum as an image:

```bash
target/release/rotation --output examples/fft.png lorem-7.5.png
```

Which looks like this:

![FFT spectrum](https://github.com/travisbrown/rotation-rs/blob/main/examples/fft.png?raw=true)
