import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";

// React v18
const root = ReactDOM.createRoot(document.getElementById("root"));
// strict mode will render or components twice in order to find certain bugs
// and also React will check if we're using outdated parts of the React API
root.render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
