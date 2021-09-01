/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 *
 * (c)2021 ZeroTier, Inc.
 * https://www.zerotier.com/
 */

import React from 'react';
import { EuiPanel, EuiText, EuiHorizontalRule } from '@elastic/eui';

export default class About extends React.Component {
    constructor(props) {
        super(props);
        this.state = {};
    }

    render() {
        return (
            <EuiPanel borderRadius="none" hasShadow={false} hasBorder={false} paddingSize="m" color="subdued">
                <EuiText>
                    <h2>ZeroTier Network Virtualization Service</h2>
                    GUI Control Application<br/>
                    (c) ZeroTier, Inc.
                </EuiText>
                <EuiHorizontalRule/>
            </EuiPanel>
        );
    }
}
