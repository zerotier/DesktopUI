import '@babel/polyfill';
import React from 'react';
import ReactDOM from 'react-dom';

import Main from './main';
import Join from './join';
import About from './about';

window.oncontextmenu = function() { return false; }

window.extLog = (data) => {
    external.invoke(JSON.stringify({
        cmd: "log",
        data: JSON.stringify(data)
    }));
};

window.onerror = function(message, source, lineno, colno, error) {
    if (message)
        extLog(message);
};

window.copyToClipboard = (str) => {
    external.invoke(JSON.stringify({
        cmd: "copy_to_clipboard",
        data: str
    }));
};

window.ztPost = (path, data) => {
    external.invoke(JSON.stringify({
        cmd: "post",
        name: path,
        data: JSON.stringify(data)
    }));
};

window.ztDelete = (path) => {
    external.invoke(JSON.stringify({
        cmd: "delete",
        name: path
    }));
};

// NOTE: window.zt_ui_update is set by primary React controls like Main. It's
// called from Rust code during polling if things have changed. This is a dummy
// for modes that don't need these updates.
window.zt_ui_update = (update) => {};

// Called from Rust code in response to 'ready' command indicating that UI should render.
window.zt_ui_render = (ui_mode, frameless) => {
    let topbar = (frameless) ? (
        <div style={{width: '100%', height: '30px'}}>
            <button style={{border: 0, padding: '2px 2px 4px 2px', width: '30px', height: '30px', textAlign: 'center', fontSize: '24px', float: 'right', display: 'inline-block'}} onClick={() => {
                external.invoke('{"cmd": "quit"}');
            }}>╳</button>
        </div>
    ) : null;
    if (ui_mode == "Main") {
        ReactDOM.render((
            <div style={{width: '100%', height: '100%'}}>
                {topbar}
                <Main/>
            </div>
        ), document.getElementById("_app_root"));
    } else if (ui_mode == "About") {
        ReactDOM.render((
            <div style={{width: '100%', height: '100%'}}>
                {topbar}
                <About/>
            </div>
        ), document.getElementById("_app_root"));
    }
    setInterval(function() { external.invoke('{ "cmd": "poll" }'); }, 200);
};

setTimeout(function() { external.invoke('{ "cmd": "ready" }'); }, 5);
