/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 *
 * (c)2021 ZeroTier, Inc.
 * https://www.zerotier.com/
 */

import React from 'react';
import { EuiFlexGroup, EuiFlexItem } from '@elastic/eui';
import equal from 'fast-deep-equal';

import ConfigPanel from './configpanel';
import NetworkList from './networklist';

export default class Main extends React.Component {
    constructor(props) {
        super(props);
        this.state = { zt: {} };
        this.onZtUpdate = this.onZtUpdate.bind(this);
        window.zt_ui_update = this.onZtUpdate;
    }

    onZtUpdate(update) {
        if (update.status) {
            // This field constantly changes and prevents equality check optimizations in React code.
            delete update.status.clock;
        }
        if (!equal(update, this.state.zt))
            this.setState({ zt: update });
    }

    render() {
        let zt = this.state.zt;
        return (
            <EuiFlexGroup className="eui-fullHeight" gutterSize="none" responsive={false}>
                <EuiFlexItem className="eui-fullHeight" grow={1} responsive={false}><ConfigPanel status={zt.status}/></EuiFlexItem>
                <EuiFlexItem className="eui-fullHeight" grow={2} responsive={false}><NetworkList networks={zt.network} savedNetworks={zt.saved_networks}/></EuiFlexItem>
            </EuiFlexGroup>
        );
    }
}
