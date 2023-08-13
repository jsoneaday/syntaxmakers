import { createBrowserRouter, RouterProvider } from "react-router-dom";
import Home from "./presentation/pages/Home";
import Developer from "./presentation/pages/Developer";

const router = createBrowserRouter([
  {
    path: "/",
    element: <Home />,
  },
  {
    path: "/developer",
    element: <Developer />,
  },
]);

function App() {
  return <RouterProvider router={router} />;
}

export default App;
