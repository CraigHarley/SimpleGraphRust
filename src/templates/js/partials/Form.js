import React, {PureComponent} from "react";

import store from "../store";
import {GET_PLAYERS, getPlayers} from "../actions";
import {connect} from "react-redux";

const styles = {
    selectWrapper: {
        padding: '1rem 0',
        display: 'flex',
        width: '80%'
    },
    button: {
        width: '100%',
        backgroundColor: '#2ECC40',
        marginTop: '2rem'
    }
};


class Form extends PureComponent {
    componentDidMount() {
        this.props.getPlayers();
    }

    render() {
        const options = store.getState().players
            .sort((a, b) => {
                if (a.name < b.name) {
                    return -1;
                }
                if (a.name > b.name) {
                    return 1;
                }
                return 0;
            })
            .map((player) =>
                <option
                    key={player.id}
                    value={player.id}
                >
                    {player.name} ({player.dob})
                </option>
            );

        return (
            <div>
                <div style={styles.selectWrapper}>
                    <i className="fas fa-user" style={{padding: '1rem'}}/>
                    <select
                        name="First"
                        id="first"
                        required
                        defaultValue={542}
                        style={{width: '20rem'}}
                    >
                        <option value="" disabled>Select first player</option>
                        {options}
                    </select>
                </div>

                <div style={styles.selectWrapper}>
                    <i className="fas fa-user" style={{padding: '1rem'}}/>
                    <select
                        name="Second"
                        id="second"
                        required
                        defaultValue={9457}
                        style={{width: '20rem'}}
                    >
                        <option value="" disabled>Select second player</option>
                        {options}
                    </select>
                </div>

                <button style={styles.button}>
                    Find
                </button>
            </div>
        );
    }
}

function mapStateToProps({players}) {
    return {
        players,
    };
}

function mapDispatchToProps(dispatch) {
    return {
        getPlayers: () => getPlayers(dispatch),
    };
}

export default connect(mapStateToProps, mapDispatchToProps)(Form);