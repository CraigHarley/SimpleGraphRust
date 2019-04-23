'use strict';
import * as ReactDOM from "react-dom";
import * as React from "react";
import "@babel/polyfill";

import Index from "./pages/index";

const e = React.createElement;
const domContainer = document.querySelector('#app');

ReactDOM.render(e(Index), domContainer);
