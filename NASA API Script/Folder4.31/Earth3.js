const req = require('request');
for (var i = 1; i < 13; i++) {
    let key = "PjWSE5ll6plyRFlYWJNC2lhBu16mMkViRAVcdJGI";
    let cloud = "False";
    let lon = "-110.83819";
    let lat = "44.525049";
    let day = "0" + i.toString();
    if (day.length > 2) {
        day = day.split("").splice(day.length-2, 2).join("");
    }
    let date = "2017-" + day + "-10";
    // console.log(date);
    let link = 'https://api.nasa.gov/planetary/earth/imagery/?lon='+ lon +'&lat='+ lat + '&date='+ date + '&cloud_score='+ cloud + '&api_key=' + key;

    req(link, { json: true }, (err, res, body) => {
        if (err) { 
            return console.log(err); 
        }
        console.log(body.url);
    });
}