#!/usr/bin/env node
const addon = require('../native');
// fs without promises is kinda cursed ngl
const fs = require('fs/promises')

if(process.argv.length > 2) {
  for(let fileName of process.argv.slice(2)) {
    fs.readFile(fileName)
    .then(file => {
      const output = addon.minify(file.toString())
      console.log(`FILE: ${fileName}\n`)
      console.log(output)
    })
  }
} else {
  console.log(`Welcome to crisp!
  Pass a file as an argument and I'll handle it for you`)
}