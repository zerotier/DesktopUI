import React, { Fragment } from 'react';
import { EuiPanel, EuiEmptyPrompt, EuiBasicTable, EuiBottomBar, EuiText, EuiHorizontalRule } from '@elastic/eui';

export const NETWORK_TABLE_COLUMNS = [
    {
        field: 'id',
        name: 'Network ID',
        sortable: true,
        width: '12rem',
        render: (id) => (
            <span className="font-monospaced">{id}</span>
        )
    },
    {
        field: 'type',
        name: 'Private',
        sortable: true,
        width: '5rem',
        render: (type) => (
            (type == "PRIVATE") ? '✓' : ' '
        )
    },
    {
        field: 'name',
        name: 'Name',
        sortable: true,
        truncateText: true
    }
];

export default class Main extends React.Component {
    constructor(props) {
        super(props);
        this.state = { networks: [] };
        this.getRowProps = this.getRowProps.bind(this);
    }

    componentWillReceiveProps(nextProps) {
        this.setState({networks: (Array.isArray(nextProps.networks)) ? nextProps.networks : []});
    }

    getRowProps(item) {
        return {};
    }

    render() {
        let networks = this.state.networks;
        let content = null;
        if ((Array.isArray(networks))&&(networks.length > 0)) {
            content = (
                <EuiPanel borderRadius="none" hasShadow={false} hasBorder={false} className="eui-fullHeight">
                    <EuiBasicTable items={networks} rowHeader="id" columns={NETWORK_TABLE_COLUMNS} tableLayout="custom" rowProps={(item) => { return this.getRowProps(item); }}/>
                </EuiPanel>
            );
        } else {
            content = (
                <EuiPanel borderRadius="none" hasShadow={false} hasBorder={false} color="subdued" className="eui-fullHeight">
                    <EuiEmptyPrompt title={<h3>You have not joined any networks.</h3>} body={
                        <Fragment>
                            <p>
                                To join a network obtain a network ID from <a>my.zerotier.com</a>, a <a>self-hosted controller</a>, or someone else who is inviting you to join their network.
                            </p>
                        </Fragment>
                    }/>
                </EuiPanel>
            );
        }
        return (
            <EuiPanel borderRadius="none" paddingSize="none" hasShadow={false} hasBorder={false} className="eui-fullHeight eui-yScroll">
                {content}
                <EuiBottomBar position="sticky" affordForDisplacement={false} paddingSize="none" landmarkHeading="Join network">
                    <EuiHorizontalRule margin="none"/>
                    <EuiPanel paddingSize="m" hasShadow={false} hasBorder={false} borderRadius="none" className="eui-fullHeight">
                        <EuiText>foo</EuiText>
                    </EuiPanel>
                </EuiBottomBar>
            </EuiPanel>
        );
    }
}
