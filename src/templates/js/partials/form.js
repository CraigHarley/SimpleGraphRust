import React from "react";

import players from "../data/players";

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

const options = players
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

export default (
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