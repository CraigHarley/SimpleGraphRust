import React, {PureComponent} from 'react';

import Head from 'next/head';

import form from "../partials/form";
import results from "../partials/results";

const styles = {
    container: {
        padding: '1rem',
        textAlign: 'center',
        display: 'flex',
        flexDirection: 'column',
        justifyContent: 'center',
        alignItems: 'center'
    }
};

export default class extends PureComponent {
    state = {
        view: this.props.view
    };

    render() {
        return (
            <div style={{padding: '1rem'}}>
                <h1 style={{textAlign: 'center'}}><i className="fas fa-project-diagram"/> Six Degrees Of Peter Crouch
                </h1>
                <div style={styles.container}>
                    {this.state.view === 'form' && form}
                    {this.state.view === 'results' && results}
                </div>
            </div>
        );
    }
}