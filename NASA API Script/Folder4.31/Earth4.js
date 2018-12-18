const req = require('request');

let key = "PjWSE5ll6plyRFlYWJNC2lhBu16mMkViRAVcdJGI";
let cloud = "False";
let lon = "-75.27832";
let lat = "-10.141932";
let date = "2017-11-15";
let link = 'https://api.nasa.gov/planetary/earth/imagery/?lon='+ lon +'&lat='+ lat + '&date='+ date + '&cloud_score='+ cloud + '&api_key=' + key;

req(link, { json: true }, (err, res, body) => {
    if (err) { 
        return console.log(err); 
    }
    console.log(body.url);
});