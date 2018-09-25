const path = require('path');
module.export = {
    entry: path.join(__dirname, 'analytics.ts'),
    output: {
        path: __dirname,
        filename: 'analytics.js',
    },
    resolve: {
        extensions: ['.ts', 'js'],
    },
    module: {
        rules: [{
            test: /\.ts$/,
            use: 'ts-loader',
        }]
    },
    mode: 'development',
}