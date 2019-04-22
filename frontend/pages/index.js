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
            <div>
                <Head>
                    <title>Six Degrees Of Peter Crouch</title>
                    <meta name="viewport" content="initial-scale=1.0, width=device-width"/>
                    <link rel='stylesheet' href='https://cdn.jsdelivr.net/gh/kognise/water.css@latest/dist/dark.css'/>
                    <link rel="stylesheet" href="https://use.fontawesome.com/releases/v5.8.1/css/all.css"
                          integrity="sha384-50oBUHEmvpQ+1lW4y57PTFmhCaXp0ML5d60M1M7uH2+nqUivzIebhndOJK28anvf"
                          crossOrigin="anonymous"/>
                    />
                </Head>
                <div style={{padding: '1rem'}}>
                    <h1 style={{textAlign: 'center'}}><i className="fas fa-project-diagram"/> Six Degrees Of Peter Crouch</h1>
                    <div style={styles.container}>
                        {this.state.view === 'form' && form}
                        {this.state.view === 'results' && results}
                    </div>
                </div>
            </div>
        );
    }
}