import React from 'react';
import { EuiPanel, EuiFlexGrid, EuiFlexItem, EuiFormRow, EuiText, EuiSpacer, EuiFieldText, EuiCheckbox, EuiButton, EuiLink, EuiSplitPanel } from '@elastic/eui';

export default class ConfigPanel extends React.Component {
    constructor(props) {
        super(props);
        this.onPrimaryPortChange = this.onPrimaryPortChange.bind(this);
        this.onPrimaryPortLostFocus = this.onPrimaryPortLostFocus.bind(this);
        this.onEnablePortMappingChange = this.onEnablePortMappingChange.bind(this);
        this.state = {
            primaryPort: null,
            portMappingEnabled: null,
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

    onEnablePortMappingChange(e) {
        this.setState({ portMappingEnabled: !this.state.portMappingEnabled });
    }

    render() {
        let status = this.props.status;
        let inner = <div></div>;
        if (status) {
            let changes = 
                (this.state.primaryPort != this.props.status?.config?.settings?.primaryPort) ||
                (this.state.portMappingEnabled != this.props.status?.config?.settings?.portMappingEnabled);
            inner = (
                <EuiSplitPanel.Outer grow={true} className="eui-fullHeight" hasShadow={false} hasBorder={false} borderRadius="none">
                    <EuiSplitPanel.Inner paddingSize="none" color="subdued">
                        <EuiFlexGrid columns={2} gutterSize="s" alignItems="center">
                            <EuiFlexItem><EuiText>ZeroTier Address</EuiText></EuiFlexItem>
                            <EuiFlexItem>
                                <EuiText>
                                    <EuiLink className="font-monospaced" color="text" onClick={ () => { copyToClipboard(status.address) } }>{status.address}</EuiLink>
                                </EuiText>
                            </EuiFlexItem>
                            <EuiFlexItem><EuiText>Version</EuiText></EuiFlexItem>
                            <EuiFlexItem><EuiText>{status.version}</EuiText></EuiFlexItem>
                            <EuiFlexItem><EuiText>Status</EuiText></EuiFlexItem>
                            <EuiFlexItem><EuiText>{status.online ? (status.tcpFallbackActive ? 'Tunneled' : 'Online') : 'Offline'}</EuiText></EuiFlexItem>

                            <EuiFlexItem><EuiSpacer/></EuiFlexItem>
                            <EuiFlexItem><EuiSpacer/></EuiFlexItem>

                            <EuiFlexItem><EuiText>Primary Port</EuiText></EuiFlexItem>
                            <EuiFlexItem>
                                <EuiFormRow helpText="(service restart required)">
                                    <EuiFieldText value={this.state.primaryPort} onBlur={(e) => { this.onPrimaryPortLostFocus(e) }} onChange={(e) => { this.onPrimaryPortChange(e) }}/>
                                </EuiFormRow>
                            </EuiFlexItem>
                            <EuiFlexItem><EuiText>Port Mapping (uPnP)</EuiText></EuiFlexItem>
                            <EuiFlexItem>
                                <EuiFormRow>
                                    <EuiCheckbox checked={this.state.portMappingEnabled} label="Enabled" onChange={(e) => { this.onEnablePortMappingChange(e) }}/>
                                </EuiFormRow>
                            </EuiFlexItem>
                        </EuiFlexGrid>
                    </EuiSplitPanel.Inner>
                    {changes ? (
                        <EuiSplitPanel.Inner grow={false} paddingSize="none" color="subdued">
                            <EuiSpacer size="m"/>
                            <EuiFlexGrid columns={2} gutterSize="s" alignItems="center">
                                <EuiFlexItem><EuiButton size="s" color="text">Cancel</EuiButton></EuiFlexItem>
                                <EuiFlexItem><EuiButton size="s" color="text" fill>Apply</EuiButton></EuiFlexItem>
                            </EuiFlexGrid>
                        </EuiSplitPanel.Inner>
                    ) : null}
                </EuiSplitPanel.Outer>
            );
        }
        return (
            <EuiPanel borderRadius="none" hasShadow={false} hasBorder={false} paddingSize="m" color="subdued">
                {inner}
            </EuiPanel>
        );
    }
}
