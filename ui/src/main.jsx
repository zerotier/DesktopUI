import React from 'react';
import { EuiPanel, EuiSpacer, EuiPageTemplate, EuiFlexGroup, EuiFlexItem } from '@elastic/eui';

import ConfigPanel from './configpanel';

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
                    <EuiFlexItem className="eui-fullHeight">
                        <ConfigPanel status={zt.status}/>
                    </EuiFlexItem>
                    <EuiFlexItem className="eui-fullHeight">
                        <EuiPanel color="subdued" borderRadius="none" hasShadow={false}>
                            <div>test 1</div>
                        </EuiPanel>
                    </EuiFlexItem>
                    <EuiFlexItem className="eui-fullHeight">
                        <EuiPanel color="subdued" borderRadius="none" hasShadow={false}>
                            <div>test 2</div>
                        </EuiPanel>
                    </EuiFlexItem>
                </EuiFlexGroup>
            </div>
        );
    }
}
