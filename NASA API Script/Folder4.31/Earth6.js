const req = require('request');
let num = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
for (var i = 1; i < 13; i++) {
    let key = "PjWSE5ll6plyRFlYWJNC2lhBu16mMkViRAVcdJGI";
    let cloud = "False";
    let lon = "-110.8358417";
    let lat = "32.1499889";
    let year = "2017";
    let month = "0" + i.toString();
    if (month.length > 2) {
        month = month.split("").splice(month.length-2, 2).join("");
    }
    let date = year + "-" + month + "-01";
    // console.log(date);
    let link2 = "https://api.nasa.gov/planetary/earth/assets?lon=" + lon + "&lat="+ lat + "&begin="+ year +"-"+ month +"-01&end=" + year + "-" +month+ "-" + num[i-1] + "&api_key=" + key;
    // console.log(link2);
    req(link2, { json: true }, (err, res, body) => {
        if (err) { 
            return console.log(err); 
        }
        if (body.count == "0") {console.log("No results::" + link2);} 
        else {
            // console.log(link2, body.results);
            date = body.results[0].date;
        };
    });
    let link = 'https://api.nasa.gov/planetary/earth/imagery/?lon='+ lon +'&lat='+ lat + '&date='+ date + '&cloud_score='+ cloud + '&api_key=' + key;
    req(link, { json: true }, (err, res, body) => {
        if (err) { 
            return console.log(err); 
        }
        console.log(date, body.url);
    });
}