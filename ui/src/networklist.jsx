import React, { Fragment } from 'react';
import { EuiPanel, EuiEmptyPrompt, EuiBasicTable, EuiBottomBar, EuiText, EuiHorizontalRule, EuiFieldText, EuiButton, EuiFlexGroup, EuiFlexItem, EuiLink } from '@elastic/eui';
import equal from 'fast-deep-equal';

import Network from './network';
import Join from './join';

export default class NetworkList extends React.Component {
    constructor(props) {
        super(props);
        this.state = {
            networks: [],
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
                    <EuiLink onClick={ () => { copyToClipboard(id) }} className="font-monospaced" color="text">{id}</EuiLink>
                )
            },
            {
                field: 'type',
                name: 'Private',
                width: '5rem',
                render: (type) => (
                    (type.toLowerCase() == 'private') ? '✓' : ' '
                )
            },
            {
                field: 'name',
                name: 'Name',
                textOnly: true,
                truncateText: true
            }
            {
                width: '3rem',
                isExpander: true,
                render: (item) => {
                    return (
                        <EuiText size="xs">
                            <EuiLink style={{fontSize: '16pt'}} aria-label={(!!this.itemIdToExpandedRowMap[item.id]) ? 'Collapse' : 'Expand'}>{(!!this.itemIdToExpandedRowMap[item.id]) ? '↑' : '↓'}</EuiLink>
                        </EuiText>
                    );
                },
            },
        ];
    }

    componentWillReceiveProps(nextProps) {
        if (!equal(this.props, nextProps)) {
            let n = nextProps.networks;
            if (Array.isArray(n)) {
                // If networks change, ensure that expanded network panels also change.
                for(let i=0;i<n.length;++i) {
                    if (this.itemIdToExpandedRowMap[n[i].id])
                        this.itemIdToExpandedRowMap[n[i].id] = <Network config={n[i]}/>;
                }
                this.setState({ networks: n, initialized: true });
            } else {
                this.setState({ networks: [], initialized: true });
            }
        }
    }

    shouldComponentUpdate(nextProps, nextState) {
        return !equal(nextState, this.state);
    }

    toggleExpand(item) {
        if (!this.itemIdToExpandedRowMap[item.id]) {
            this.itemIdToExpandedRowMap[item.id] = <Network config={item}/>;
        } else {
            delete this.itemIdToExpandedRowMap[item.id];
        }
        this.forceUpdate();
    }

    getRowProps(item) {
        return {
            onClick: () => { this.toggleExpand(item) }
        };
    }

    render() {
        let networks = this.state.networks;

        let content = null;
        if (this.state.initialized) {
            if ((Array.isArray(networks))&&(networks.length > 0)) {
                content = <EuiBasicTable
                    items={networks}
                    itemId='id'
                    itemIdToExpandedRowMap={this.itemIdToExpandedRowMap}
                    isExpandable={true}
                    isSelectable={false}
                    responsive={false}
                    rowHeader="id"
                    rowProps={this.getRowProps}
                    columns={this.networkTableColumns}
                    tableLayout="custom"
                />;
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
            return <div></div>;
        }
    }
}
