/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 *
 * (c)2021 ZeroTier, Inc.
 * https://www.zerotier.com/
 */

import React from 'react';
import { EuiPanel, EuiFlexGroup, EuiFlexItem, EuiFormRow, EuiCheckbox, EuiLink, EuiFlexGrid, EuiText, EuiSpacer, EuiButton } from '@elastic/eui';

export default class Network extends React.Component {
    constructor(props) {
        super(props);
        this.adiToIp = this.adiToIp.bind(this);
        this.state = {};
    }

    adiToIp(adi) {
        return ((adi >> 24) & 0xff).toString() + '.' + ((adi >> 16) & 0xff).toString() + '.' + ((adi >> 8) & 0xff).toString() + '.' + (adi & 0xff).toString();
    }

    render() {
        let config = this.props.config;
        return (
            <EuiFlexGroup gutterSize="none" responsive={false}>
                <EuiFlexItem>
                    <EuiFlexGrid columns={2} gutterSize="none" responsive={false}>
                        <EuiFlexItem><EuiText size="s">ID</EuiText></EuiFlexItem>
                        <EuiFlexItem><EuiLink style={{color: 'inherit'}} onClick={() => { copyToClipboard(config.id, "Copied network ID to clipboard.") }} size="s">{config.id}</EuiLink></EuiFlexItem>
                        <EuiFlexItem><EuiText size="s">Name</EuiText></EuiFlexItem>
                        <EuiFlexItem><EuiLink style={{color: 'inherit'}} onClick={() => { copyToClipboard(config.name, "Copied network name to clipboard.") }} size="s">{config.name}</EuiLink></EuiFlexItem>
                        <EuiFlexItem><EuiText size="s">Type</EuiText></EuiFlexItem>
                        <EuiFlexItem><EuiText size="s">{(config.private) ? 'PRIVATE' : 'PUBLIC'}</EuiText></EuiFlexItem>
                        <EuiFlexItem><EuiText size="s">Status</EuiText></EuiFlexItem>
                        <EuiFlexItem><EuiText size="s">{config.status}</EuiText></EuiFlexItem>
                        <EuiFlexItem><EuiText size="s">Ethernet MAC</EuiText></EuiFlexItem>
                        <EuiFlexItem><EuiLink style={{color: 'inherit'}} onClick={() => { copyToClipboard(config.mac, "Copied virtual MAC to clipboard.") }} className="font-monospaced" size="s">{config.mac}</EuiLink></EuiFlexItem>
                        <EuiFlexItem><EuiText size="s">Virtual NIC Device</EuiText></EuiFlexItem>
                        <EuiFlexItem><EuiLink style={{color: 'inherit'}} onClick={() => { copyToClipboard(config.portDeviceName, "Copied port name to clipboard.") }} size="s">{config.portDeviceName}</EuiLink></EuiFlexItem>
                        <EuiFlexItem><EuiText size="s">Virtual NIC MTU</EuiText></EuiFlexItem>
                        <EuiFlexItem><EuiText size="s">{config.mtu}</EuiText></EuiFlexItem>
                        <EuiFlexItem><EuiText size="s">Ethernet Broadcast</EuiText></EuiFlexItem>
                        <EuiFlexItem><EuiText size="s">{config.broadcastEnabled ? 'enabled' : 'disabled'}</EuiText></EuiFlexItem>
                        <EuiFlexItem><EuiText size="s">Ethernet Bridging</EuiText></EuiFlexItem>
                        <EuiFlexItem><EuiText size="s">{config.bridge ? 'allowed' : 'prohibited'}</EuiText></EuiFlexItem>
                        <EuiFlexItem><EuiText size="s">DNS Domain</EuiText></EuiFlexItem>
                        <EuiFlexItem><EuiText size="s">{(((config.dns||{}).domain) ? config.dns.domain : '(not configured)')}</EuiText></EuiFlexItem>
                        <EuiFlexItem><EuiText size="s">DNS Servers</EuiText></EuiFlexItem>
                        <EuiFlexItem><EuiText size="s">{((((config.dns||{}).servers||[]).length > 0) ? config.dns.servers.map((server) => { return <span key={server}>{server}</span>; }) : '(none)')}</EuiText></EuiFlexItem>

                        <EuiFlexItem><EuiSpacer size="s"/></EuiFlexItem>
                        <EuiFlexItem><EuiSpacer size="s"/></EuiFlexItem>

                        <EuiFlexItem><EuiText size="s">Allow Managed IPs</EuiText></EuiFlexItem>
                        <EuiFlexItem><EuiCheckbox compressed={true} checked={config.allowManaged} onChange={(e) => {
                            window.ztPost('network/' + config.id, { allowManaged: !config.allowManaged });
                        }}/></EuiFlexItem>
                        <EuiFlexItem><EuiText size="s">Allow Global Internet IPs</EuiText></EuiFlexItem>
                        <EuiFlexItem><EuiCheckbox compressed={true} checked={config.allowGlobal} onChange={(e) => {
                            window.ztPost('network/' + config.id, { allowGlobal: !config.allowGlobal });
                        }}/></EuiFlexItem>
                        <EuiFlexItem><EuiText size="s">Allow Default Route Override</EuiText></EuiFlexItem>
                        <EuiFlexItem><EuiCheckbox compressed={true} checked={config.allowDefault} onChange={(e) => {
                            window.ztPost('network/' + config.id, { allowDefault: !config.allowDefault });
                        }}/></EuiFlexItem>
                        <EuiFlexItem><EuiText size="s">Allow DNS Configuration</EuiText></EuiFlexItem>
                        <EuiFlexItem><EuiCheckbox compressed={true} checked={config.allowDNS} onChange={(e) => {
                            window.ztPost('network/' + config.id, { allowDNS: !config.allowDNS });
                        }}/></EuiFlexItem>
                    </EuiFlexGrid>
                </EuiFlexItem>
                <EuiFlexItem>
                    <EuiFormRow label="Managed IPs">
                        <div style={{overflow: 'hidden'}}>
                            <EuiPanel className="eui-yScroll" paddingSize="s" border={false} hasShadow={false} style={{height: 'calc(4em + 8px)'}}>
                                <EuiText size="xs">
                                    {(config.assignedAddresses||[]).map((address) => {
                                        return <div className="font-monospaced" key={address}>{address}</div>;
                                    })}
                                </EuiText>
                            </EuiPanel>
                        </div>
                    </EuiFormRow>
                    <EuiFormRow label="Managed Routes">
                        <div style={{overflow: 'hidden'}}>
                            <EuiPanel className="eui-yScroll" paddingSize="s" border={false} hasShadow={false} style={{height: 'calc(4em + 8px)'}}>
                                <EuiText size="xs" style={{whiteSpace: 'nowrap', overflowX: 'hidden'}}>
                                    {(config.routes||[]).map((route) => {
                                        let via = route.via||'(lan)';
                                        return <div className="font-monospaced" key={(route.target+via)}>{route.target} via {via}</div>;
                                    })}
                                </EuiText>
                            </EuiPanel>
                        </div>
                    </EuiFormRow>
                    <EuiFormRow label="Ethernet Multicast Subscriptions">
                        <div style={{overflow: 'hidden'}}>
                            <EuiPanel className="eui-yScroll" paddingSize="s" border={false} hasShadow={false} style={{height: 'calc(4em + 8px)'}}>
                                <EuiText size="xs" style={{whiteSpace: 'nowrap', overflowX: 'hidden'}}>
                                    {(config.multicastSubscriptions||[]).map((ms) => {
                                        let adi = 0;
                                        try {
                                            adi = parseInt(ms.adi);
                                        } catch (e) {}
                                        adi = (adi > 0) ? ((ms.mac === 'ff:ff:ff:ff:ff:ff') ? ' (ARP ADI: '+this.adiToIp(adi)+')' : ' (non-ARP ADI: '+adi.toString(16)+')') : '';
                                        return <div className="font-monospaced" key={(ms.mac+adi)}>{ms.mac}{adi}</div>;
                                    })}
                                </EuiText>
                            </EuiPanel>
                        </div>
                    </EuiFormRow>
                    <EuiSpacer size="m"/>
                    <div className="eui-textRight">
                        <EuiButton color="danger" size="s" onClick={() => {
                            if (typeof config.id === 'string' && config.id.length === 16) {
                                ztRememberNetwork(config.id, config.name||'', config);
                                ztDelete('network/' + config.id);
                            }
                        }}>Disconnect</EuiButton>
                    </div>
                </EuiFlexItem>
            </EuiFlexGroup>
        );
    }
}
