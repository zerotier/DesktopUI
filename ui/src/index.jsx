import React, { createElement } from 'react';
import ReactDOM from 'react-dom';

//import { EuiPanel, EuiPageTemplate, EuiText, EuiResizableContainer } from '@elastic/eui';

import Main from './main'

// Called from Rust code to return data requested with 'get'.
window.zt_get_callbacks = {};
window.zt_get_callback = (callback_name, data) => {
    let cb = window.zt_get_callbacks[callback_name];
    delete window.zt_get_callbacks[callback_name];
    if (typeof cb === 'function') {
        try {
            cb(data);
        } catch (e) {}
    }
};

// Called by React controls to obtain or refresh their state.
// NOTE: for testing this could be replaced with a mock version that calls
// the callback with test data.
window.zt_get = (path, callback) => {
    let callback_name = Math.random().toString() + Math.random().toString();
    window.zt_get_callbacks[callback_name] = callback;
    external.invoke(JSON.stringify({
        cmd: "get",
        name: path, // path relative to local service API base e.g. http://127.0.0.1:9993/<path>
        data: callback_name // random name of entry in window.zt_get_callbacks[]
    }));
};

// Called from Rust code in response to 'ready' command indicating that UI should render.
window.zt_ui_render = (ui_mode) => {
    if (ui_mode == "Main") {
        ReactDOM.render(<Main/>, document.getElementById("_app_root"));
    } else if (ui_mode == "Join") {
        ReactDOM.render((<div>window_type = {window_type}</div>), document.getElementById("_app_root"));
    } else if (ui_mode == "About") {
        ReactDOM.render((<div>window_type = {window_type}</div>), document.getElementById("_app_root"));
    } else {
        ReactDOM.render((<div>unrecognized ui_mode = {ui_mode}</div>), document.getElementById("_app_root"));
    }
};

setTimeout(function() { external.invoke('{ "cmd": "ready" }'); }, 5);
