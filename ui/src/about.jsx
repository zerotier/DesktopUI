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
