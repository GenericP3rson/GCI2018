var GIFEncoder = require('gifencoder');
var encoder = new GIFEncoder(512, 512);
var pngFileStream = require('png-file-stream');
var fs = require('fs');
 
pngFileStream('img?.png')
  .pipe(encoder.createWriteStream({ repeat: -1, delay: 500, quality: 10 }))
  .pipe(fs.createWriteStream('myanimated.gif'));