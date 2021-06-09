import React from 'react';

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
		return (
			<div>zt: {JSON.stringify(this.state.zt)}</div>
		);
	}
}
