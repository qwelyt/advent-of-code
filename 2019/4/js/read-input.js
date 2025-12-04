const fs = require('fs');
const path = require('path');
const filePath = path.join(__dirname, '../input.txt');

const file = fs.readFileSync(filePath, {encoding: 'UTF-8'}, function(err, data){
  if(err){
    console.log(err);
  } else {
    return data;
  }
});

module.exports = file;