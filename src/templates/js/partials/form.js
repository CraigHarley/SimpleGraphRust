import React, {PureComponent} from "react";
import VirtualList from 'react-tiny-virtual-list';

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

const data = ['A', 'B', 'C', 'D', 'E', 'F'];

export default class extends PureComponent {
    componentDidMount() {
        this.props.getPlayers();
    }

    render() {
        return (
            <div>
                <div style={styles.selectWrapper}>
                    <i className="fas fa-user" style={{padding: '1rem'}}/>
                    <VirtualList
                        width='100%'
                        height={600}
                        itemCount={data.length}
                        itemSize={50} // Also supports variable heights (array or function getter)
                        renderItem={({index, style}) =>
                            <div key={index} style={style}> // The style property contains the item's absolute position
                                Letter: {data[index]}, Row: #{index}
                            </div>
                        }
                    />
                </div>

                <div style={styles.selectWrapper}>
                    <i className="fas fa-user" style={{padding: '1rem'}}/>
                    <VirtualList
                        width='100%'
                        height={600}
                        itemCount={data.length}
                        itemSize={50} // Also supports variable heights (array or function getter)
                        renderItem={({index, style}) =>
                            <div key={index} style={style}> // The style property contains the item's absolute position
                                Letter: {data[index]}, Row: #{index}
                            </div>
                        }
                    />
                </div>

                <button style={styles.button}>
                    Find
                </button>
            </div>
        );
    }
}