import store from '../store';

import {UPDATE_PLAYERS_REDUCER, UPDATE_RESULTS_REDUCER} from "../reducers";

export const GET_PLAYERS = 'GET_PLAYERS';
export const GET_RESULTS = 'GET_RESULTS';

export const getPlayers = async () => {

    console.log(players);
    store.commit(UPDATE_PLAYERS_REDUCER, players);
};

export const getResults = async () => {
    console.log(players);
    store.commit(UPDATE_RESULTS_REDUCER, players);
}

export default {
    [GET_PLAYERS]: getPlayers,
    [GET_RESULTS]: getResults
}