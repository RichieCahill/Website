
const { readFileSync, writeFileSync } = require('fs');

const express = require('express');
const app = express();

const { readFile } = require('fs');

app.use(express.static('public'));
app.use('/images', express.static(__dirname + '/Images'));




app.get('/',(reg, res) => {
  readFile('./index.html', 'utf-8', (err, html) => {

    if(err){
      res.status(500).send('Sorry, out of order')
    }

    res.send(html);
  })
});

app.get('/about', function (req, res) {
  readFile('./about.html', 'utf-8', (err, html) => {

    if(err){
      res.status(404).send('page not found')
    }

    res.send(html);
  })
});

app.get('/resume', function (req, res) {
  readFile('./resume.html', 'utf-8', (err, html) => {

    if(err){
      res.status(404).send('page not found')
    }

    res.send(html);
  })
});

app.get('/Mistakes', function (req, res) {
  readFile('./Mistakes.html', 'utf-8', (err, html) => {

    if(err){
      res.status(404).send('page not found')
    }

    res.send(html);
  })
});

app.listen(5000, () => console.log('http://localhost:5000/'))

