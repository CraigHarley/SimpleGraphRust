import {GET_PLAYERS, GET_RESULTS} from "../reducers";

const GET_PLAYERS_ACTION = (payload) => ({
    type: GET_PLAYERS,
    payload
});

export const getPlayers = async (dispatch) => {
    const response = await fetch('/players');
    dispatch(GET_PLAYERS_ACTION(await response.json()));
};

const GET_RESULTS_ACTION = (payload) => ({
    type: GET_RESULTS,
    payload
});

export const getResults = async (dispatch) => {
    // todo request
    dispatch(GET_RESULTS_ACTION({}));
};

export default {
    [GET_PLAYERS]: getPlayers,
    [GET_RESULTS]: getResults
}