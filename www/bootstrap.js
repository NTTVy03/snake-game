// use bootstrap.js instead of call index.js directly in config
// to print error message if import failed
import("./index.js").catch(e => console.error("Error importing index.js: ", e))
