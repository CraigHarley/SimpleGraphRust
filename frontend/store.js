import {createStore} from 'redux'
import reducers from './reducers';
const initialState = {};

const rootReducer = (state = initialState, reducer) => {
    if (reducers[reducer.type]) {
        return reducers[reducer.type](state, reducer.payload)
    }

    console.log('reducer not found.');
    return state
};

export default createStore(rootReducer);