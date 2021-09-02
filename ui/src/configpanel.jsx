/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 *
 * (c)2021 ZeroTier, Inc.
 * https://www.zerotier.com/
 */

import React from 'react';
import { EuiPanel, EuiFlexGrid, EuiFlexItem, EuiFormRow, EuiText, EuiSpacer, EuiFieldText, EuiCheckbox, EuiButton, EuiLink, EuiSplitPanel } from '@elastic/eui';

export default class ConfigPanel extends React.Component {
    constructor(props) {
        super(props);
        this.onPrimaryPortChange = this.onPrimaryPortChange.bind(this);
        this.onPrimaryPortLostFocus = this.onPrimaryPortLostFocus.bind(this);
        this.onEnablePortMappingChange = this.onEnablePortMappingChange.bind(this);
        this.apply = this.apply.bind(this);
        this.cancel = this.cancel.bind(this);
        this.hasChanges = this.hasChanges.bind(this);
        this.state = {
            primaryPort: null,
            portMappingEnabled: null,
            receivedProps: false,
            submittedChanges: false,
            needsRestart: false
        };
    }

    componentWillReceiveProps(nextProps) {
        let primaryPort = nextProps.status?.config?.settings?.primaryPort;
        let portMappingEnabled = nextProps.status?.config?.settings?.portMappingEnabled;
        if ((!this.state.receivedProps)&&(!this.state.submittedChanges)) {
            this.setState({
                primaryPort: primaryPort,
                portMappingEnabled: portMappingEnabled,
                receivedProps: true,
                submittedChanges: false
            });
        }
    }

    onPrimaryPortChange(e) {
        try {
            let vstr = e.target.value||'0';
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

    apply() {
        this.setState({
            submittedChanges: true,
            needsRestart: true
        });
        ztPost('config/settings', {
            primaryPort: this.state.primaryPort,
            portMappingEnabled: !!this.state.portMappingEnabled
        });
    }

    cancel() {
        let primaryPort = this.props.status?.config?.settings?.primaryPort;
        let portMappingEnabled = this.props.status?.config?.settings?.portMappingEnabled;
        this.setState({
            primaryPort: primaryPort,
            portMappingEnabled: portMappingEnabled
        });
    }

    hasChanges() {
        return (this.state.primaryPort != this.props.status?.config?.settings?.primaryPort) ||
               (this.state.portMappingEnabled != this.props.status?.config?.settings?.portMappingEnabled);
    }

    render() {
        let status = this.props.status;
        let inner = <div/>;
        if (status) {
            inner = (
                <EuiSplitPanel.Outer responsive={false} grow={true} className="eui-fullHeight" hasShadow={false} hasBorder={false} borderRadius="none">
                    <EuiSplitPanel.Inner paddingSize="none" color="subdued" responsive={false}>
                        <EuiFlexGrid columns={2} gutterSize="s" alignItems="center" responsive={false}>
                            <EuiFlexItem><EuiText>ZeroTier Address</EuiText></EuiFlexItem>
                            <EuiFlexItem grow={false}>
                                <EuiText>
                                    <EuiLink className="font-monospaced" color="text" onClick={ () => { copyToClipboard(status.address, "Copied this node's ZeroTier address to clipboard.") } }>{status.address}</EuiLink>
                                </EuiText>
                            </EuiFlexItem>
                            <EuiFlexItem><EuiText>Version</EuiText></EuiFlexItem>
                            <EuiFlexItem grow={false}><EuiText>{status.version}</EuiText></EuiFlexItem>
                            <EuiFlexItem><EuiText>Status</EuiText></EuiFlexItem>
                            <EuiFlexItem grow={false}><EuiText>{status.online ? (status.tcpFallbackActive ? 'Tunneled' : 'Online') : 'Offline'}</EuiText></EuiFlexItem>

                            <EuiFlexItem grow={false}><EuiSpacer size="m"/></EuiFlexItem>
                            <EuiFlexItem grow={false}><EuiSpacer size="m"/></EuiFlexItem>

                            <EuiFlexItem><EuiText>Primary Port</EuiText></EuiFlexItem>
                            <EuiFlexItem grow={false}>
                                <EuiFormRow helpText="(service restart required)">
                                    <EuiFieldText style={{width: '5rem'}} value={this.state.primaryPort} onBlur={(e) => { this.onPrimaryPortLostFocus(e) }} onChange={(e) => { this.onPrimaryPortChange(e) }}/>
                                </EuiFormRow>
                            </EuiFlexItem>
                            <EuiFlexItem><EuiText>Port Mapping (uPnP)</EuiText></EuiFlexItem>
                            <EuiFlexItem grow={false}>
                                <EuiFormRow>
                                    <EuiCheckbox checked={this.state.portMappingEnabled} label="Enabled" onChange={(e) => { this.onEnablePortMappingChange(e) }}/>
                                </EuiFormRow>
                            </EuiFlexItem>
                        </EuiFlexGrid>
                    </EuiSplitPanel.Inner>
                    {this.hasChanges() ? (
                        <EuiSplitPanel.Inner grow={false} paddingSize="none" color="subdued" responsive={false}>
                            <EuiSpacer size="m"/>
                            <EuiText size="s">Note: changes to ports or port mapping require the ZeroTier service to be restarted.</EuiText>
                            <EuiSpacer size="m"/>
                            <EuiFlexGrid columns={2} gutterSize="s" alignItems="center" responsive={false}>
                                <EuiFlexItem><EuiButton size="s" color="text" onClick={() => { this.cancel(); }}>Cancel</EuiButton></EuiFlexItem>
                                <EuiFlexItem><EuiButton size="s" color="text" fill onClick={() => { this.apply(); }}>Apply</EuiButton></EuiFlexItem>
                            </EuiFlexGrid>
                        </EuiSplitPanel.Inner>
                    ) : ((this.state.needsRestart) ? (
                        <EuiSplitPanel.Inner grow={false} paddingSize="none" color="subdued" responsive={false}>
                            <EuiSpacer size="m"/>
                            <EuiText size="s">Changes written to local.conf but require the ZeroTier service to be restarted to take effect.</EuiText>
                        </EuiSplitPanel.Inner>
                    ) : null)}
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
