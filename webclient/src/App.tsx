import { createBrowserRouter, RouterProvider } from "react-router-dom";
import Home from "./presentation/pages/Home";
import DeveloperJobs from "./presentation/pages/DeveloperJobs";
import JobFullView from "./presentation/pages/JobFullView";

const router = createBrowserRouter([
  {
    path: "/",
    element: <Home />,
  },
  {
    path: "/developerjobs",
    element: <DeveloperJobs />,
  },
  {
    path: "/jobfullview",
    element: <JobFullView />,
  },
]);

function App() {
  return <RouterProvider router={router} />;
}

export default App;
