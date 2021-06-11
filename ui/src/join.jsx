import React from 'react';
import { EuiFlexGroup, EuiFlexItem, EuiFormRow, EuiFieldText, EuiButton } from '@elastic/eui';

export default class Join extends React.Component {
    constructor(props) {
        super(props);
        this.state = { joinNetworkId: '' };
        this.onJoinNetworkChanged = this.onJoinNetworkChanged.bind(this);
    }

    onJoinNetworkChanged(e) {
        try {
            let vstr = e.target.value||'';
            let s = '';
            for(let i=0;(i<vstr.length)&&(i<16);++i) {
                let c = vstr.charAt(i);
                if ("0123456789abcdefABCDEF".indexOf(c) >= 0)
                    s += c;
            }
            this.setState({ joinNetworkId: s.toLowerCase() });
        } catch (exc) {
            this.setState({ joinNetworkId: '' });
        }
    }

    render() {
        return (
            <div style={{ height: this.props.height, width: this.props.width, padding: '15px' }}>
                <EuiFlexGroup gutterSize="m" responsive={false}>
                    <EuiFlexItem grow={false}>
                        <EuiFormRow>
                            <EuiFieldText value={this.state.joinNetworkId} placeholder="################" style={{width: '12em'}} className="font-monospaced" onChange={(e) => { this.onJoinNetworkChanged(e); }}/>
                        </EuiFormRow>
                    </EuiFlexItem>
                    <EuiFlexItem>
                        <EuiButton isDisabled={((this.state.joinNetworkId||'').length !== 16)} color="text" fill>Join&nbsp;Network</EuiButton>
                    </EuiFlexItem>
                </EuiFlexGroup>
            </div>
        );
    }
}
