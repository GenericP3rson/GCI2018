var fs      = require('fs');
var request = require('request');
// Or with cookies
// var request = require('request').defaults({jar: true});

request.get({url: 'https://earthengine.googleapis.com/api/thumb?thumbid=8c93b69a066dfd1bfc38a77b355f3293&token=42fd7445be895b74cb256d1975099a9b', encoding: 'binary'}, function (err, response, body) {
  fs.writeFile("hello.png", body, 'binary', function(err) {
    if(err)
      console.log(err);
    else
      console.log("The file was saved!");
  }); 
});