export const UPDATE_PLAYERS_REDUCER = 'UPDATE_PLAYERS_REDUCER';
export const UPDATE_RESULTS_REDUCER = 'UPDATE_RESULTS_REDUCER';

export const updatePlayersReducer = (state, players) => ({
    ...state,
    players
});

export const updateResultsReducer = (state, results) => ({
    ...state,
    results
});

export default {
    [UPDATE_PLAYERS_REDUCER]: updatePlayersReducer,
    [UPDATE_RESULTS_REDUCER]: updateResultsReducer
};
