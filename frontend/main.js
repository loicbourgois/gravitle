import * as Index from './index.js';
import * as Playground from './playground.js';

const url = new URL(window.location.href);

if (url.pathname.includes('index.html')) {
    Index.start();
} else if (url.pathname.includes('playground.html')) {
    Playground.start();
} else {
    Index.start();
}
