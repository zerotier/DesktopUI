import React, { createElement } from 'react';

export default class Main extends React.Component {
    constructor(props) {
        super(props);
        this.state = { ss: {} };
    }

	render() {
		return (
			<div>state: {this.state.ss}</div>
		);
	}
};
