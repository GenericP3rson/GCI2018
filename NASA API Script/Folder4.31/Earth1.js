const req = require('request');
var key = "PjWSE5ll6plyRFlYWJNC2lhBu16mMkViRAVcdJGI";
req('https://api.nasa.gov/planetary/apod?api_key=PjWSE5ll6plyRFlYWJNC2lhBu16mMkViRAVcdJGI', { json: true }, (err, res, body) => {
    if (err) { 
        return console.log(err); 
    }
    console.log(body.url);
    console.log(body.title);
    console.log(body.explanation);
    console.log(body.hdurl);
});