import React, {PureComponent} from 'react';
import { Provider } from 'react-redux';

import results from "../partials/results";
import store from "../store";
import Form from "../partials/Form";

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
        view: 'form'
    };

    render() {
        return (
            <Provider store={store}>
                <div style={{padding: '1rem'}}>
                    <h1 style={{textAlign: 'center'}}>
                        <i className="fas fa-project-diagram"/> Six Degrees Of Peter Crouch
                    </h1>
                    <div style={styles.container}>
                        {this.state.view === 'form' && <Form/>}
                        {this.state.view === 'results' && results}
                    </div>
                </div>
            </Provider>
        );
    }
}