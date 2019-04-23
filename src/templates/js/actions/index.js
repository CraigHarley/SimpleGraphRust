import store from '../store';

import {UPDATE_PLAYERS_REDUCER, UPDATE_RESULTS_REDUCER} from "../reducers";

export const GET_PLAYERS = 'GET_PLAYERS';
export const GET_RESULTS = 'GET_RESULTS';

export const getPlayers = async () => {
    const response = await fetch('/players');
    store.commit(UPDATE_PLAYERS_REDUCER, response.json());
};

export const getResults = async (payload) => {
    store.commit(UPDATE_RESULTS_REDUCER, payload);
};

export default {
    [GET_PLAYERS]: getPlayers,
    [GET_RESULTS]: getResults
}