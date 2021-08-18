import '@babel/polyfill';
import React from 'react';
import ReactDOM from 'react-dom';

import Main from './main';
import About from './about';

window.oncontextmenu = function() { return false; }

window.addEventListener("keypress", (event) => {
    if ((event.metaKey||event.altKey) && ((event.key === 'q')||(event.key === 'x'))) {
        external.invoke('{"cmd": "quit"}');
        event.preventDefault();
    }
});

window.extLog = (data) => {
    external.invoke(JSON.stringify({
        cmd: "log",
        data: JSON.stringify(data)
    }));
};

window.onerror = function(message, source, lineno, colno, error) {
    extLog(message);
};

window.copyToClipboard = (str) => {
    external.invoke(JSON.stringify({
        cmd: "copy_to_clipboard",
        data: str
    }));
};

window.zt_paste_callback_function = null;
window.zt_paste_from_clipboard_callback = (data) => {
    if (typeof window.zt_paste_callback_function === 'function') {
        if (Array.isArray(data)) {
            let s = "";
            for (let i=0;i<data.length;++i) {
                s += String.fromCharCode(data[i]);
            }
            window.zt_paste_callback_function(s);
        } else {
            window.zt_paste_callback_function('');
        }
    }
    window.zt_paste_callback_function = null;
};
window.pasteFromClipboard = (callback) => {
    window.zt_paste_callback_function = callback;
    external.invoke('{"cmd": "paste_from_clipboard"}');
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

window.ztRememberNetwork = (networkId, networkName, networkSettings) => {
    external.invoke(JSON.stringify({
        cmd: "remember_network",
        name: networkId,
        data: networkName,
        data2: networkSettings
    }));
};

window.ztForgetNetwork = (networkId) => {
    external.invoke(JSON.stringify({
        cmd: "forget_network",
        name: networkId
    }));
};

// NOTE: window.zt_ui_update is set by primary React controls like Main. It's
// called from Rust code during polling if things have changed. This is a dummy
// for modes that don't need these updates.
window.zt_ui_update = (update) => {};

// Called from Rust code in response to 'ready' command indicating that UI should render.
window.zt_ui_render = (ui_mode, frameless) => {
    if (ui_mode === "Main") {
        ReactDOM.render((<div style={{width: '100%', height: '100%'}}><Main/></div>), document.getElementById("_app_root"));
    } else if (ui_mode === "About") {
        ReactDOM.render((<div style={{width: '100%', height: '100%'}}><About/></div>), document.getElementById("_app_root"));
    }
    setInterval(function() { external.invoke('{ "cmd": "poll" }'); }, 200);
    setTimeout(function() { external.invoke('{ "cmd": "raise" }'); }, 1200);
};

setTimeout(function() { external.invoke('{ "cmd": "ready" }'); }, 5);
