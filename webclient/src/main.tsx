import ReactDOM from "react-dom/client";
import App from "./App.tsx";
import "./presentation/theme/base.css";
import { Provider as ReduxProvider } from "react-redux";
import { store } from "./presentation/common/redux/Store.ts";

ReactDOM.createRoot(document.getElementById("root")!).render(
  <ReduxProvider store={store}>
    <App />
  </ReduxProvider>
);
