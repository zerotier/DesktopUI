import React from 'react';
import { EuiPanel, EuiFlexGrid, EuiFlexItem, EuiFormRow, EuiText, EuiHorizontalRule, EuiFieldText, EuiCheckbox, EuiButton } from '@elastic/eui';

export default class ConfigPanel extends React.Component {
    constructor(props) {
        super(props);
        this.onPrimaryPortChange = this.onPrimaryPortChange.bind(this);
        this.onPrimaryPortLostFocus = this.onPrimaryPortLostFocus.bind(this);
        this.state = {
            primaryPort: null,
            portMappingEnabled: null,
            changes: false,
            receivedProps: false
        };
    }

    componentWillReceiveProps(nextProps) {
        let primaryPort = nextProps.status?.config?.settings?.primaryPort;
        let portMappingEnabled = nextProps.status?.config?.settings?.portMappingEnabled;
        if (!this.state.receivedProps) {
            this.setState({
                primaryPort: primaryPort,
                portMappingEnabled: portMappingEnabled,
                receivedProps: true
            });
        } else {
            this.setState({
                changes: primaryPort!=this.state.primaryPort || portMappingEnabled!=this.state.portMappingEnabled
            });
        }
    }

    onPrimaryPortChange(e) {
        try {
            let vstr = e.target.value;
            let v = parseInt(vstr, 10);
            if ((v)&&(v > 0)&&(v < 65536)) {
                this.setState({ primaryPort: v });
            } else if (!vstr) {
                this.setState({ primaryPort: '' });
            }
        } catch (exc) {}
    }

    onPrimaryPortLostFocus(e) {
        let vstr = e.target.value;
        if (!vstr) {
            this.setState({ primaryPort: (((this.props.status||{}).config||{}).settings||{}).primaryPort });
        }
    }

    render() {
        let status = this.props.status;
        let inner = <div></div>;
        if (status) {
            inner = (<>
                <EuiFlexGrid columns={2} gutterSize="s" alignItems="center">
                    <EuiHorizontalRule/>
                    <EuiFlexItem><EuiText><h4>ZeroTier Address</h4></EuiText></EuiFlexItem>
                    <EuiFlexItem><EuiText><span className="font-monospaced">{status.address}</span></EuiText></EuiFlexItem>
                    <EuiFlexItem><EuiText><h4>Version</h4></EuiText></EuiFlexItem>
                    <EuiFlexItem><EuiText>{status.version}</EuiText></EuiFlexItem>
                    <EuiFlexItem><EuiText><h4>Status</h4></EuiText></EuiFlexItem>
                    <EuiFlexItem><EuiText>{status.online ? (status.tcpFallbackActive ? 'Tunneled' : 'Online') : 'Offline'}</EuiText></EuiFlexItem>
                    <EuiHorizontalRule/>
                    <EuiFlexItem><EuiText><h4>Primary Port</h4></EuiText></EuiFlexItem>
                    <EuiFlexItem>
                        <EuiFormRow helpText="(service restart required)">
                            <EuiFieldText value={this.state.primaryPort} onBlur={(e) => { this.onPrimaryPortLostFocus(e) }} onChange={(e) => { this.onPrimaryPortChange(e) }}/>
                        </EuiFormRow>
                    </EuiFlexItem>
                    <EuiFlexItem><EuiText><h4>Port Mapping (uPnP)</h4></EuiText></EuiFlexItem>
                    <EuiFlexItem>
                        <EuiFormRow>
                            <EuiCheckbox checked={this.state.portMappingEnabled} label="Enabled" onChange={(e) => { this.setState({portMappingEnabled: !this.state.portMappingEnabled}) }}/>
                        </EuiFormRow>
                    </EuiFlexItem>
                    <EuiHorizontalRule/>
                    {(this.state.changes) ? (<>
                        <EuiFlexItem>
                            <EuiButton size="s" color="text">Cancel</EuiButton>
                        </EuiFlexItem>
                        <EuiFlexItem>
                            <EuiButton size="s" color="text">Apply</EuiButton>
                        </EuiFlexItem>
                    </>) : null}
                </EuiFlexGrid>
            </>);
        }
        return (
            <EuiPanel borderRadius="none" hasShadow={false} hasBorder={false}>
                {inner}
            </EuiPanel>
        );
    }
}
