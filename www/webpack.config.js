const path = require("path");
const CopyWebpackPlugin = require("copy-webpack-plugin");

// copy index.js file to the public/index.js
// and run the server in development mode
module.exports = {
    entry: "./bootstrap.js",
    output: {
        path: path.resolve(__dirname, "public"),
        filename: "bootstrap.js"
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