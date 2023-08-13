import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App.tsx";
import "./presentation/theme/base.css";
import { Provider as ReduxProvider } from "react-redux";
import { store } from "./presentation/common/redux/Store.ts";
import ReactModal from "react-modal";

ReactModal.setAppElement("#root");

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <ReduxProvider store={store}>
      <App />
    </ReduxProvider>
  </React.StrictMode>
);
