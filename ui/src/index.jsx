import React from 'react';
import ReactDOM from 'react-dom';

import Main from './main';
import Join from './join';
import About from './about';

window.extLog = (data) => {
    external.invoke(JSON.stringify({
        cmd: "log",
        data: JSON.stringify(data)
    }));
};

window.copyToClipboard = (str) => {
    external.invoke(JSON.stringify({
        cmd: "copy_to_clipboard",
        data: str
    }));
};

// Sends a message to the Rust code to be passed through as a POST to the service's JSON API.
window.ztPost = (path, data) => {
    external.invoke(JSON.stringify({
        cmd: "post",
        name: path,
        data: JSON.stringify(data)
    }));
};

// NOTE: window.zt_ui_update is set by primary React controls like Main. It's
// called from Rust code during polling if things have changed. This is a dummy
// for modes that don't need these updates.
window.zt_ui_update = (update) => {};

// Called from Rust code in response to 'ready' command indicating that UI should render.
window.zt_ui_render = (ui_mode) => {
    if (ui_mode == "Main") {
        ReactDOM.render(<Main/>, document.getElementById("_app_root"));
    } else if (ui_mode == "About") {
        ReactDOM.render(<About/>, document.getElementById("_app_root"));
    } else {
        ReactDOM.render((<div>unrecognized ui_mode = {ui_mode}</div>), document.getElementById("_app_root"));
    }

    setInterval(function() { external.invoke('{ "cmd": "poll" }'); }, 200);
};

setTimeout(function() { external.invoke('{ "cmd": "ready" }'); }, 5);
