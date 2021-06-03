import React, { createElement } from 'react';
import ReactDOM from 'react-dom';

import { EuiPanel, EuiPageTemplate, EuiText, EuiResizableContainer } from '@elastic/eui';

window.zt_ui_render = function(window_type) {
    if (window_type == "MainWindow") {
        ReactDOM.render((
            <EuiResizableContainer style={{ height: "100vh", maxHeight: "100vh", minHeight: "100vh"}}>{(EuiResizablePanel, EuiResizableButton) => (<>
                <EuiResizablePanel initialSize={50} minSize="30%" grow={true}>
                    <EuiText>
                        panel 1
                    </EuiText>
                </EuiResizablePanel>
                <EuiResizableButton />
                <EuiResizablePanel initialSize={50} minSize="30%" grow={true}>
                    <EuiText>
                        panel 2
                    </EuiText>
                </EuiResizablePanel>
            </>)}</EuiResizableContainer>
        ), document.getElementById("_app_root"));
    } else if (window_type == "JoinNetwork") {
        ReactDOM.render((
            <div>window_type = {window_type}</div>
        ), document.getElementById("_app_root"));
    } else {
        ReactDOM.render((
            <div>window_type = {window_type}</div>
        ), document.getElementById("_app_root"));
    }
};

// Tell Rust code JS has initialized and React app is now running.
setTimeout(function() { external.invoke('{ "cmd": "ready" }'); }, 1);
