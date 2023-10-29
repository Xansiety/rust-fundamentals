# Rust + WebAssembly + Webpack

It's necessary create a new project with cargo of type lib

```bash
cargo init --lib
```

After  create the package.json file

```bash
npm init -y
```
Before, install Webpack and Webpack CLI

```bash
npm i -D webpack webpack-cli webpack-dev-server
```

Create the `webpack.config.js` file in the root of the project
and configure the entry point and the output file.
By default webpack not support HTML files, so it's necessary install the plugin `html-webpack-plugin`

```bash
npm i -D html-webpack-plugin
```


Add a new command in the `package.json` file to run the webpack server

```json
"dev": "webpack serve --mode=development"
```

for build the project

```json
"build": "webpack --mode=production"
```


Add a WASM to the project

```bash
npm install -D @wasm-tool/wasm-pack-plugin
```

This package will be used to compile the Rust code to WASM and add it to the project.

Update the `webpack.config.js` file to add the plugin and add the plugin to the plugins array


Now, go to the `cargo.toml` file and add the following lines

```toml
[lib]
crate-type = ["cdylib"]
```


For comunicate the Rust code with the JavaScript code, it's necessary add the `wasm-bindgen` package

in the cargo.toml file add the following lines

```toml
[dependencies]
wasm-bindgen = "0.2.87"
```

For new versions, its necessary enable `asyncWebAssembly` to call the Rust functions from JavaScript in async mode
go to `webpack.config.js` and add the following lines

```js
experiments: {
    asyncWebAssembly: true
}
```

now compile the Rust code to WASM

```bash
npx webpack
```

run the server

```bash
npm run dev
```

