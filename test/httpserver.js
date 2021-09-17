
const fs = require('fs');
const express = require('express');

const addon = require('..');

const app = express();
const port = 3000;

app.get('/img', (req, res) => {
    const start = Date.now();
    addon.imageGen({imgx: 1000, imgy: 1000}, function(err, buf) {
        console.log('-=-=',Date.now() - start);
        res.setHeader('content-type', 'image/jpg');
        res.end(buf);
    });
})

app.listen(port, () => {
    console.log('server start', process.pid);
});

// setInterval(() => {
//     console.log('--', Date.now());
// }, 500)
