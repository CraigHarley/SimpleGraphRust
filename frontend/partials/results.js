import React from "react";

const styles = {
    ul: {
        listStyle: 'none'
    },
    button: {
        width: '100%',
        backgroundColor: '#2ECC40',
        marginTop: '2rem'
    }
};

export default (
    <div>
        <form action="">
            <ul style={styles.ul}>
                <li>Blah Blah Blah</li>
                <li>Blah Blah Blah</li>
                <li>Blah Blah Blah</li>
                <li>Blah Blah Blah</li>
            </ul>
            <button style={styles.button}>
                Try Another
            </button>
        </form>
    </div>
);
