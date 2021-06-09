import React from 'react';
import ReactDOM from 'react-dom';

//import { EuiPanel, EuiPageTemplate, EuiText, EuiResizableContainer } from '@elastic/eui';

import Main from './main'

// Send a log to stdout from the UI process.
window.extLog = (data) => {
    external.invoke(JSON.stringify({
        cmd: "log",
        data: JSON.stringify(data)
    }));
};

// NOTE: window.zt_ui_update is set by primary React controls like Main. It's
// called from Rust code during polling if things have changed.

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
setInterval(function() { external.invoke('{ "cmd": "poll" }'); }, 250);
