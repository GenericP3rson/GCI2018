const req = require('request');
const fs = require("fs");
let q = 1;
let num = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
for (var i = 1; i < 13; i++) {
    let key = "PjWSE5ll6plyRFlYWJNC2lhBu16mMkViRAVcdJGI"; // My Key
    let cloud = "False"; // Whether or not to show cloud percentage
    let lon = "-38.519932"; // Longitude
    let lat = "-3.72168"; // Latitude
    let year = "2014"; 
    let month = "0" + i.toString();
    if (month.length > 2) {
        month = month.split("").splice(month.length-2, 2).join("");
    }
    let date = year + "-" + month + "-01"; // The Date
    let link2 = "https://api.nasa.gov/planetary/earth/assets?lon=" + lon + "&lat=" + lat + "&begin=" + year + "-" + month + "-01&end=" + year + "-" + month+ "-" + num[i-1] + "&api_key=" + key;
    // Sets the link to retrieve which day has data.
    req(link2, { json: true }, (err, res, body) => {
        if (err) { 
            return console.log(err); 
        }
        if (body.count == "0") {console.log("No results::" + link2);} 
        else {
            date = body.results[0].date;
            // Will set the day to the first day with data
        };
    });
    let link = 'https://api.nasa.gov/planetary/earth/imagery/?lon='+ lon + '&lat=' + lat + '&date=' + date + '&cloud_score=' + cloud + '&api_key=' + key;
    // The link to get the actual photos.
    req(link, { json: true }, (err, res, body) => {
        if (err) { 
            return console.log(err); 
        }
        console.log(date, body.url);
        req.get({url: body.url, encoding: 'binary'}, function (err, response, body) {
        fs.writeFile("img"+ q +".png", body, 'binary', function(err) {
            if(err) console.log(err);
            else {console.log("The file was saved!" + q); q++; }
        }); 
        });
    });
}