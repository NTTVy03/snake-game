# Set up web-pack

## 1. What is web-pack

Webpack is a popular module bundler for JavaScript applications. It compiles JavaScript modules and other resources (like CSS, images, and more) into a single or a few output files, which are optimized and ready for deployment to the web.

### Key Features of web-pack

* **Bundling:** Combines multiple JavaScript files (modules) into fewer output files (bundles) for optimized loading and performance.
* **Dependency Management:** Automatically manages dependencies between modules, resolving them in the correct order.
* **Code Splitting:** Splits your code into smaller chunks that can be loaded on demand, improving load times by only loading necessary code.
* **Loaders and Plugins:** Supports processing and transforming files (e.g., compiling SCSS to CSS, converting ES6+ JavaScript to ES5 with Babel, and more).
* **Asset Management:** Bundles assets such as images, fonts, and styles along with JavaScript, creating a unified output.

### Example Workflow

1. **Setup Webpack Config:** Define how assets should be processed (like JavaScript, CSS, images).
2. **Run Webpack to Bundle:** Webpack scans your entry points, builds dependency graphs, and bundles your code.
3. **Deploy Optimized Bundle:** Use the generated output files (bundles) for production deployment.

## 2. How to set up web-pack for JS

For example, `www/` will contains our project with structure:
```
www/
    ├── index.html       
    └── index.js       
```

1. Init project: fast init a Node.js project (generate `package.json` file)
    ```
    // -y accepts all default value
    npm init -y
    ```
2. Download Node.js: [nodejs](https://nodejs.org/en/download/prebuilt-installer). Type `node` in terminal to test if download success
3. Install `webpack` and `webpack-cli` (run webpack commands from the command line)
    ```
    // --save: add them to `dependencies` in package.json
    // do not need for `npm` version 5+
    npm install --save webpack webpack-cli
    ```
4. Install `webpack-dev-server` (provides a live reloading environment for faster development)
    ```
    // --save-dev: add them to `devDependencies` in package.json
    npm install --save-dev webpack-dev-server
    ```
5. Install `copy-webpack-plugin` (copy files or entire directories from one location to another during the build process)
    ```
    npm install copy-webpack-plugin
    ```
6. Ignore `node_modules/`: create `.gitignore` file and add `node_modules/` into it
7. Create file `webpack.config.js`
    ```javascripts=
    const path = require("path");
    const CopyWebpackPlugin = require("copy-webpack-plugin");

    // copy index.js and index.html to the public/
    // before build/run the server in development mode
    module.exports = {
        entry: "./index.js",
        output: {
            path: path.resolve(__dirname, "public"),
            filename: "index.js"
        },
        mode: "development",
        plugins: [
            new CopyWebpackPlugin({
                patterns: [
                    { from: "./index.html", to: "./" }
                ]
            })
        ]
    }
    ```
10. Config `package.json`
    ```
    "scripts": {
        "test": "echo \"Error: no test specified\" && exit 1",
        // automatically re-run if any change occurs
        "dev": "webpack-dev-server", 
        "build": "webpack build"
      },
    ```
11. Run/build this project: 
    ```
    // this command will find command in 
    // scripts/dev in package.json, and run it
    npm run dev
    ```
    or
    ```
    npm run build
    ```
    
## 3. What is typecsripts (TS)

TypeScript is designed to be a superset of JavaScript, meaning that:
* JavaScript code is TypeScript code: You can take an existing JavaScript file, rename it to `.ts`, and it will work as TypeScript. Over time, you can add types and interfaces as needed.
* Gradual Typing: TypeScript allows you to add types gradually, meaning you can start with plain JavaScript and only add types where you see the benefit. This makes it easier to adopt TypeScript in existing JavaScript projects.

Example:
* JavaScript Code:
    ```javascript=
    function greet(name) {
      return "Hello, " + name;
    }
    ```
* TypeScript Code (With Type Annotations):
    ```typescript=
    function greet(name: string): string {
      return "Hello, " + name;
    }
    ```

## 4. How to set up web-pack for TS
> [TypeScript in Webpack](https://webpack.js.org/guides/typescript/)


