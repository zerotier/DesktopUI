/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 *
 * (c)2021 ZeroTier, Inc.
 * https://www.zerotier.com/
 */

import React from 'react';
import { EuiFlexGroup, EuiFlexItem, EuiFormRow, EuiFieldText, EuiButton } from '@elastic/eui';

export default class Join extends React.Component {
    constructor(props) {
        super(props);
        this.state = { joinNetworkId: '' };
        this.onJoinNetworkChangedSet = this.onJoinNetworkChangedSet.bind(this);
        this.onJoinNetworkChanged = this.onJoinNetworkChanged.bind(this);
        //this.onJoinNetworkKeyPress = this.onJoinNetworkKeyPress.bind(this);
    }

    onJoinNetworkChangedSet(newValue) {
        let s = '';
        for(let i=0;(i<newValue.length)&&(i<16);++i) {
            let c = newValue.charAt(i);
            if ("0123456789abcdefABCDEF".indexOf(c) >= 0)
                s += c;
        }
        this.setState({ joinNetworkId: s.toLowerCase() });
    }

    onJoinNetworkChanged(e) {
        this.onJoinNetworkChangedSet(e.target.value||'');
    }

    /*
    onJoinNetworkKeyPress(e) {
        if ((e.metaKey||e.ctrlKey) && e.key === 'v') {
            e.preventDefault();
            window.pasteFromClipboard((data) => {
                if (typeof data === 'string') {
                    this.onJoinNetworkChangedSet(data);
                }
            });
        }
        if ((e.metaKey||e.ctrlKey) && e.key === 'c') {
            e.preventDefault();
            if (e.target.value) {
                window.copyToClipboard(e.target.value);
            }
        }
    }
    */

    render() {
        return (
            <div style={{ height: this.props.height, width: this.props.width, padding: '15px' }}>
                <EuiFlexGroup gutterSize="m" responsive={false}>
                    <EuiFlexItem grow={false}>
                        <EuiFormRow><EuiFieldText
                            autofocus={true}
                            controlOnly={true}
                            value={this.state.joinNetworkId}
                            placeholder="################"
                            style={{width: '12em'}}
                            className="font-monospaced"
                            /*onKeyPress={(e) => { this.onJoinNetworkKeyPress(e) }}*/
                            onChange={(e) => { this.onJoinNetworkChanged(e) }}
                        /></EuiFormRow>
                    </EuiFlexItem>
                    <EuiFlexItem>
                        <EuiButton isDisabled={((this.state.joinNetworkId||'').length !== 16)} color="text" fill onClick={() => {
                            ztPost('network/' + this.state.joinNetworkId, {});
                            this.setState({ joinNetworkId: '' });
                        }}>Join&nbsp;Network</EuiButton>
                    </EuiFlexItem>
                </EuiFlexGroup>
            </div>
        );
    }
}
