const ex = require('express');
const bp = require('body-parser');
const app = ex();
app.use(bp.text());
app.post('*', (req, res) => {
    console.log('request from ', req.originalUrl);
    console.log('client addr: ', req.headers['x-client-address']);
    console.log('body: ', req.body);
    res.send();
});

app.listen(5555, err => {
    if (err) throw err;
    console.log('listening on 5555');
});