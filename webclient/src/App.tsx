import "./App.css";
import { createBrowserRouter, RouterProvider } from "react-router-dom";
import Home from "./presentation/pages/Home";

const router = createBrowserRouter([
  {
    path: "/",
    element: <Home />,
  },
]);

function App() {
  return <RouterProvider router={router} />;
}

export default App;
