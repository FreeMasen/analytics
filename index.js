const ex = require('express');
const app = ex();

app.post('*', (req, res) => {
    console.log('request from ', req.originalUrl);
    console.log('client addr: ', req.headers['x-client-address']);
    console.log('body: ', req.body);
    res.send();
})