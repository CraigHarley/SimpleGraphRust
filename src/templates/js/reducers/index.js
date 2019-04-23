export const GET_PLAYERS = 'GET_PLAYERS';
export const GET_RESULTS = 'GET_RESULTS';

export const getPlayersReducer = (state, players) => ({
    ...state,
    players
});

export const getResultsReducer = (state, results) => ({
    ...state,
    results
});

export default {
    [GET_PLAYERS]: getPlayersReducer,
    [GET_RESULTS]: getResultsReducer
};
