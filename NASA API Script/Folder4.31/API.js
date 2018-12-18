const https = require('https');

var web = ["https://api.nasa.gov/neo/rest/v1/feed?start_date=2015-09-07&end_date=2015-09-08&api_key=DEMO_KEY", "https://api.nasa.gov/neo/rest/v1/neo/browse?api_key=DEMO_KEY"]

for (var i = 0; i < web.length; i++) {
    https.get(web[i], (resp) => {
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
}


// https.get('https://api.nasa.gov/planetary/apod?api_key=DEMO_KEY', (resp) => {
//   let data = '';

//   // A chunk of data has been recieved.
//   resp.on('data', (chunk) => {
//     data += chunk;
//   });

//   // The whole response has been received. Print out the result.
//   resp.on('end', () => {
//     console.log(JSON.parse(data).explanation);
//   });

// }).on("error", (err) => {
//   console.log("Error: " + err.message);
// });