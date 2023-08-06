import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App.tsx";
import "./presentation/theme/base.css";
import { Provider as ReduxProvider } from "react-redux";
import { store } from "./domain/redux/Store.ts";

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <ReduxProvider store={store}>
      <App />
    </ReduxProvider>
  </React.StrictMode>
);
