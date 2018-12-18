const req = require('request');

req('https://api.nasa.gov/planetary/earth/imagery?lon=100.75&lat=1.5&date=2014-02-01&cloud_score=True&api_key=DEMO_KEY', { json: true }, (err, res, body) => {
    if (err) { 
        return console.log(err); 
    // self.callback(err, null);
    // return;
    }
    console.log(body.url);
    console.log(body.explanation);
});