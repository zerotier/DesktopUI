import React from 'react';
import { EuiPanel, EuiSpacer, EuiPageTemplate, EuiFlexGroup, EuiFlexItem } from '@elastic/eui';
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
                <EuiFlexItem className="eui-fullHeight" grow={2} responsive={false}><NetworkList networks={zt.network}/></EuiFlexItem>
            </EuiFlexGroup>
        );
    }
}
