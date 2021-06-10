import React from 'react';
import { EuiPanel, EuiSpacer, EuiPageTemplate, EuiFlexGroup, EuiFlexItem } from '@elastic/eui';

import ConfigPanel from './configpanel';
import NetworkList from './networklist';

export default class Main extends React.Component {
    constructor(props) {
        super(props);
        this.state = { zt: {} };

        window.zt_ui_update = this.onZtUpdate.bind(this);
    }

    onZtUpdate(update) {
        this.setState({ zt: update });
    }

    render() {
        let zt = this.state.zt;
        return (
            <div className="eui-fullHeight">
                <EuiFlexGroup className="eui-fullHeight" gutterSize="none">
                    <EuiFlexItem className="eui-fullHeight" grow={1}>
                        <EuiPanel paddingSize="none" hasShadow={false} hasBorder={true} className="eui-fullHeight">
                            <ConfigPanel status={zt.status}/>
                        </EuiPanel>
                    </EuiFlexItem>
                    <EuiFlexItem className="eui-fullHeight" grow={2}>
                        <NetworkList networks={zt.network}/>
                    </EuiFlexItem>
                </EuiFlexGroup>
            </div>
        );
    }
}
