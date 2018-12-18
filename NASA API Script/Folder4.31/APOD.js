var apod = require("apod");
 
apod.apiKey = "PjWSE5ll6plyRFlYWJNC2lhBu16mMkViRAVcdJGI";
 
function callback(err, data) {
  // do cool stuff with APOD data here
  console.log(data);
}
 
// get today's APOD
// apod(callback);
 
// APOD for December 31, 1999 (JS has 0-indexed months)
apod(new Date(2018, 9, 31), callback);
 
// the same
// apod(1999, 11, 31, callback);
 
// once more, with feeling
// apod("December 31, 1999", callback);
 
// get a random APOD
// apod.random(callback);