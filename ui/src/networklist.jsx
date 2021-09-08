/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 *
 * (c)2021 ZeroTier, Inc.
 * https://www.zerotier.com/
 */

import React, { Fragment } from 'react';
import { EuiPanel, EuiEmptyPrompt, EuiBasicTable, EuiLink, EuiButtonEmpty } from '@elastic/eui';
import equal from 'fast-deep-equal';

import Network from './network';
import Join from './join';

export default class NetworkList extends React.Component {
    constructor(props) {
        super(props);
        this.state = {
            networks: [],
            savedNetworks: {},
            selectedRowNetworkId: '',
            initialized: false
        };
        this.itemIdToExpandedRowMap = {};
        this.toggleExpand = this.toggleExpand.bind(this);
        this.getRowProps = this.getRowProps.bind(this);

        this.networkTableColumns = [
            {
                field: 'id',
                name: 'Network ID',
                width: '12rem',
                render: (id) => (
                    <span className="font-monospaced">{id}</span>
                )
            },
            {
                name: 'Name',
                isExpander: true,
                render: (item) => {
                    if (item.status) {
                        return (
                            <div style={{width: '100%'}}>
                                <span>{item.name}</span>
                                <span style={{float: 'right'}}>
                                    <EuiLink style={{fontSize: '16pt', paddingRight: '0.5rem'}} aria-label={(!!this.itemIdToExpandedRowMap[item.id]) ? 'Collapse' : 'Expand'}>{(!!this.itemIdToExpandedRowMap[item.id]) ? '↑' : '↓'}</EuiLink>
                                </span>
                            </div>
                        );
                    } else {
                        return (
                            <div style={{width: '100%'}}>
                                <span>{item.name}</span>
                                <span style={{float: 'right'}}>
                                    <EuiButtonEmpty size="s" aria-label='Reconnect to Network' color='primary' onClick={() => { ztPost('network/' + item.id, item.settings); }}>Reconnect</EuiButtonEmpty>
                                    &nbsp;&nbsp;
                                    <EuiButtonEmpty size="s" aria-label='Forget Network' color='danger' onClick={() => { ztForgetNetwork(item.id); }}>Forget</EuiButtonEmpty>
                                </span>
                            </div>
                        );
                    }
                },
            }
        ];
    }

    componentWillReceiveProps(nextProps) {
        if (!equal(this.props, nextProps)) {
            let n = nextProps.networks;
            let sn = nextProps.savedNetworks;
            if (Array.isArray(n)) {
                // If networks change, ensure that expanded network panels also change.
                let haveNetworks = {};
                for(let i=0;i<n.length;++i) {
                    if (this.itemIdToExpandedRowMap[n[i].id])
                        this.itemIdToExpandedRowMap[n[i].id] = <Network config={n[i]}/>;
                    haveNetworks[n[i].id] = true;
                }
                let exp = Object.keys(this.itemIdToExpandedRowMap);
                for(let i=0;i<exp.length;++i) {
                    if (!haveNetworks[exp[i]]) {
                        delete this.itemIdToExpandedRowMap[exp[i]];
                    }
                }
                this.setState({ networks: n, initialized: true, savedNetworks: sn||{} });
            } else {
                this.setState({ networks: [], initialized: true, savedNetworks: sn||{} });
            }
        }
    }

    shouldComponentUpdate(nextProps, nextState) {
        return !equal(nextState, this.state);
    }

    toggleExpand(item) {
        if (item.status) {
            if (!this.itemIdToExpandedRowMap[item.id]) {
                this.itemIdToExpandedRowMap[item.id] = <Network config={item}/>;
            } else {
                delete this.itemIdToExpandedRowMap[item.id];
            }
            this.forceUpdate();
        }
    }

    getRowProps(item) {
        return {
            onClick: () => { this.toggleExpand(item) }
        };
    }

    render() {
        let networks = this.state.networks;
        let savedNetworks = this.state.savedNetworks;

        let content = null;
        if (this.state.initialized) {
            let merged = [];
            let joined = {};
            for(let i=0;i<networks.length;++i) {
                merged.push(networks[i]);
                joined[networks[i].id] = true;
            }
            let savedIds = Object.keys(savedNetworks);
            for(let i=0;i<savedIds.length;++i) {
                let o = savedNetworks[savedIds[i]];
                if ((o.id)&&(!joined[o.id]))
                    merged.push(o);
            }

            if ((Array.isArray(merged))&&(merged.length > 0)) {
                content = (
                    <EuiBasicTable
                        items={merged}
                        itemId='id'
                        itemIdToExpandedRowMap={this.itemIdToExpandedRowMap}
                        isExpandable={true}
                        isSelectable={false}
                        responsive={false}
                        rowHeader="id"
                        rowProps={this.getRowProps}
                        columns={this.networkTableColumns}
                        tableLayout="custom"
                    />
                );
            } else {
                content = (
                    <EuiEmptyPrompt title={<h3>You have not joined any networks.</h3>} body={
                        <Fragment>
                            <p>
                                To join a network obtain a network ID from <a>my.zerotier.com</a>, a <a>self-hosted controller</a>,
                                or someone else who is inviting you to join their network.
                            </p>
                        </Fragment>
                    }/>
                );
            }

            return (
                <EuiPanel borderRadius="none" paddingSize="none" hasShadow={false} hasBorder={false} className="eui-fullHeight" style={{ overflowY: 'hidden' }}>
                    <EuiPanel borderRadius="none" hasShadow={false} hasBorder={false} paddingSize="m" className="eui-yScroll" style={{ height: 'calc(100% - 70px)' }}>{content}</EuiPanel>
                    <Join height="70px" width="24em"/>
                </EuiPanel>
            );
        } else {
            return <div/>;
        }
    }
}
