<div align="center">

  <h1><code>wasm-pack-template</code></h1>

  <strong>A template for kick starting a Rust and WebAssembly project using <a href="https://github.com/rustwasm/wasm-pack">wasm-pack</a>.</strong>

  <p>
    <a href="https://travis-ci.org/rustwasm/wasm-pack-template"><img src="https://img.shields.io/travis/rustwasm/wasm-pack-template.svg?style=flat-square" alt="Build Status" /></a>
  </p>

  <h3>
    <a href="https://rustwasm.github.io/docs/wasm-pack/tutorials/npm-browser-packages/index.html">Tutorial</a>
    <span> | </span>
    <a href="https://discordapp.com/channels/442252698964721669/443151097398296587">Chat</a>
  </h3>

  <sub>Built with 🦀🕸 by <a href="https://rustwasm.github.io/">The Rust and WebAssembly Working Group</a></sub>
</div>

## About

[**📚 Read this template tutorial! 📚**][template-docs]

This template is designed for compiling Rust libraries into WebAssembly and
publishing the resulting package to NPM.

Be sure to check out [other `wasm-pack` tutorials online][tutorials] for other
templates and usages of `wasm-pack`.

[tutorials]: https://rustwasm.github.io/docs/wasm-pack/tutorials/index.html
[template-docs]: https://rustwasm.github.io/docs/wasm-pack/tutorials/npm-browser-packages/index.html

## 🚴 Usage

### 🐑 Use `cargo generate` to Clone this Template

[Learn more about `cargo generate` here.](https://github.com/ashleygwilliams/cargo-generate)

```
cargo generate --git https://github.com/rustwasm/wasm-pack-template.git --name my-project
cd my-project
```

### 🛠️ Build with `wasm-pack build`

```
wasm-pack build
```

### 🔬 Test in Headless Browsers with `wasm-pack test`

```
wasm-pack test --headless --firefox
```

### 🎁 Publish to NPM with `wasm-pack publish`

```
wasm-pack publish
```

## 🔋 Batteries Included

* [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
* [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.
* `LICENSE-APACHE` and `LICENSE-MIT`: most Rust projects are licensed this way, so these are included for you

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.


### 使用方法
```html
<!DOCTYPE html>
<html>
<head>
  <title>WASM MD5 Test</title>
</head>
<body>
<h1>WASM MD5 Test</h1>
<input type="file" id="fileInput">
<p>MD5: <span id="md5Result"></span></p>
<button >a</button>
<script type="module">
  import init, { calculate_md5, calculate_md5_from_chunks } from './pkg/wasm_file_md5.js';

  async function run() {
    await init();

    const fileInput = document.getElementById('fileInput');
    const md5Result = document.getElementById('md5Result');

    fileInput.addEventListener('change', async (event) => {
      const file = event.target.files[0];
      if (!file) return;

      try {
        const startTime = performance.now();
        const md5 = await calculateMD5WithStream(file);
        const endTime = performance.now();
        md5Result.textContent = `MD5: ${md5}, time: ${endTime - startTime}ms`;

      } catch (error) {
        console.error("Error reading file:", error);
        md5Result.textContent = "Error reading file: " + error.message;
      }
    });


    async function calculateMD5WithStream(file) {
      const stream = file.stream();
      const reader = stream.getReader();
      const chunks = [];

      try {
        while(true) {
          const { done, value } = await reader.read();
          if(done) {
            break;
          }
          chunks.push(value);
        }
        return calculate_md5_from_chunks(chunks);
      } catch (error) {
        console.error("Error reading stream:", error);
        throw error;
      } finally {
        reader.releaseLock();
      }
    }
  }

  run();
</script>
</body>
</html>

```