<div align="center">

  <img src="https://github.com/visioncortex/symcode/raw/master/docs/images/visioncortex-banner.png">
  <h1>SymCode Web App</h1>

  <p>
    <strong>The Symbolic Barcode for Humans and Machines</strong>
  </p>

  <h3>
    <a href="https://www.visioncortex.org/symcode-docs">Story</a>
    <span> | </span>
    <a href="https://symcode.visioncortex.org/">Demo</a>
    <span> | </span>
    <a href="https://github.com/visioncortex/acute32">Acute32</a>
  </h3>
  <sub>Built with 🦀 by <a href="//www.visioncortex.org/">The Vision Cortex Research Group</a></sub>
</div>

# Synopsis

Since `symcode` is a pure rust programming library, this crate glues everything together and leverage 
the browser's capability for image and video processing.

The `app` directory is the source of the demo web app.

The `dev` directory is the test bench for development.

If you only want to integrate SymCode into your Javascript project, you can simply use the wasm 
binaries provided at [`acute32`](https://github.com/visioncortex/acute32).

Note that only the `src` directory is covered under open source license, other contents are 
considered proprietary.
