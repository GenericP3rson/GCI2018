var request = require('request');

request('https://api.nasa.gov/planetary/earth/assets?lon=100.75&lat=1.5&begin=2014-02-01&api_key=DEMO_KEY', 
  function (error, response, body) {
    var data = JSON.parse(body);
    var result = data.results;
    for(var i=0; i<result.length; i++){
      console.log(result[i].date);
    }
  }
);