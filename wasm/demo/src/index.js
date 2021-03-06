import './style.css';
import 'codemirror/lib/codemirror.css';
import 'xterm/dist/xterm.css';

// A dependency graph that contains any wasm must all be imported
// asynchronously. This `index.js` file does the single async import, so
// that no one else needs to worry about it again.
import('./main.js').catch(e => {
    console.error('Error importing `main.js`:', e);
    document.getElementById('error').textContent = e;
});
