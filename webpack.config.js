const path = require('path')

module.exports = {
   context: path.resolve(__dirname, 'src'),
   entry: './index.js',
   output: {
     path: __dirname + '/dist',
     publicPath: '/',
     filename: 'bundle.js'
   },
   devServer: {
      static: {
         directory: __dirname + '/public'
       },
   },
   resolve: {
      extensions: ['.js', '.jsx'],
  },
   module: {
     rules: [
      {
         test: /\.css$/,
         use: ['style-loader', 'css-loader'],
     },
     {
       test: /\.(js|jsx)$/,
       exclude: /node_modules/,
       use: ['babel-loader']
     }
     ]
   },
 };