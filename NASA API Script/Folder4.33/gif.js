var GIFEncoder = require('gifencoder');
var Canvas = require(
const { createCanvas, loadImage } = require('canvas')
var fs = require('fs');
var encoder = new GIFEncoder(512, 512);
encoder.createReadStream().pipe(fs.createWriteStream('try.gif'));
 
encoder.start();
encoder.setRepeat(0);
encoder.setDelay(500);
encoder.setQuality(10); 

var x = 512;

var canvas = Canvas.createCanvas(320, 240);
var ctx = canvas.getContext('2d');

loadImage('examples/images/lime-cat.jpg').then((image) => {
    ctx.drawImage(image, 50, 0, 70, 70)
   
    console.log('<img src="' + canvas.toDataURL() + '" />')
  })

ctx.fillStyle = '#ff0000';
ctx.fillRect(0, 0, x, x);
encoder.addFrame(ctx);

ctx.fillStyle = '#00ff00';
ctx.fillRect(0, 0, x, x);
encoder.addFrame(ctx);

ctx.fillStyle = '#0000ff';
ctx.fillRect(0, 0, x, x);
encoder.addFrame(ctx);
 
encoder.finish();