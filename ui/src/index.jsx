/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 *
 * (c)2021 ZeroTier, Inc.
 * https://www.zerotier.com/
 */

import '@babel/polyfill';
import React from 'react';
import ReactDOM from 'react-dom';

import Main from './main';
import About from './about';

window.oncontextmenu = function() { return false; }

window.addEventListener("keypress", (event) => {
    if ((event.metaKey||event.altKey) && ((event.key === 'q')||(event.key === 'x'))) {
        window.rpc.notify('quit');
        event.preventDefault();
    }
});

window.extLog = (data) => {
    window.rpc.notify('log', data);
};

window.onerror = function(message, source, lineno, colno, error) {
    extLog(message);
};

window.copyToClipboard = (str, msg) => {
    window.rpc.notify('copy_to_clipboard', str||'')
};

window.pasteFromClipboard = (callback) => {
    window.rpc.call('paste_from_clipboard').then((data) => {
        if (Array.isArray(data)) {
            let s = "";
            for (let i=0;i<data.length;++i) {
                s += String.fromCharCode(data[i]);
            }
            callback(s);
        } else {
            callback('');
        }
    });
};

window.ztPost = (path, data) => {
    window.rpc.notify('post', [path, JSON.stringify(data)]);
};

window.ztDelete = (path) => {
    window.rpc.notify('delete', path);
};

window.ztRememberNetwork = (networkId, networkName, networkSettings) => {
    window.rpc.notify('remember_network', [networkId, networkName, JSON.stringify(networkSettings||{})]);
};

window.ztForgetNetwork = (networkId) => {
    window.rpc.notify('forget_network', networkId);
};

// NOTE: window.zt_ui_update is set by primary React controls like Main. It's
// called from Rust code during polling if things have changed. This is a dummy
// for modes that don't need these updates.
window.zt_ui_update = (update) => {};

window.zt_ui_render = (ui_mode) => {
    if (ui_mode === "Main") {
        ReactDOM.render((<div style={{width: '100%', height: '100%'}}><Main/></div>), document.getElementById("_app_root"));
        setInterval(function() {
            window.rpc.call('poll').then((result) => {
                if (result) {
                    window.zt_ui_update(JSON.parse(result));
                }
            });
        }, 200);
    } else if (ui_mode === "About") {
        ReactDOM.render((<div style={{width: '100%', height: '100%'}}><About/></div>), document.getElementById("_app_root"));
    }
};

setTimeout(function() {
    window.rpc.call('ready').then((ui_mode) => { window.zt_ui_render(ui_mode); });
}, 5);
