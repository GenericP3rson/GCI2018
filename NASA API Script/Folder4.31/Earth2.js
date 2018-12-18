const https = require('https');
https.get('https://api.nasa.gov/planetary/earth/imagery?lon=100.75&lat=1.5&date=2014-02-01&cloud_score=True&api_key=PjWSE5ll6plyRFlYWJNC2lhBu16mMkViRAVcdJGI', (resp) => {
    let data = '';
    resp.on('data', (stuff) => {
        data += stuff;
    });

    // The whole response has been received. Print out the result.
    resp.on('end', () => {
        console.log(JSON.parse(data).explanation);
    });
    }).on("error", (err) => {
    console.log("Error: " + err.message);
    });