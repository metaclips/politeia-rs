module.exports = {
    pages: {
        'home': {
            entry: './src/pages/Home/main.js',
            template: 'public/index.html',
            title: 'Home',
            chunks: ['chunk-vendors', 'chunk-common', 'index']
        },
        'proposal': {
            entry: './src/pages/Proposal/main.js',
            template: 'public/index.html',
            title: 'About',
            chunks: ['chunk-vendors', 'chunk-common', 'about']
        }
    }
}
