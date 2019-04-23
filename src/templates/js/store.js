import {createStore} from "redux";
import reducers from "./reducers";

const initialState = {
    players: [

    ],
    results: {

    }
};

// interface IStandardAction<T> {
//     type: string;
//     payload: T;
// }

function rootReducer(state = initialState, action) {
    if (reducers[action.type]) {
        return reducers[action.type](state, action.payload);
    } else {
        return state;
    }
}

const store = createStore(
    rootReducer,
    initialState,
    window.__REDUX_DEVTOOLS_EXTENSION__ && window.__REDUX_DEVTOOLS_EXTENSION__()
);
export default store;
