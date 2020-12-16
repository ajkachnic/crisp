var addon = require('../native');

console.log(addon.minify(`
  console.log(' hello world' );
`));
