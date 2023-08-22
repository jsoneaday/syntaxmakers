import { createBrowserRouter, RouterProvider } from "react-router-dom";
import Home from "./presentation/pages/Home";
import DeveloperHome from "./presentation/pages/developer/DeveloperHome";
import JobFullView from "./presentation/pages/developer/JobFullView";
import EmployerHome from "./presentation/pages/employer/EmployerHome";

export enum RoutePaths {
  Root = "/",
  DevHome = "devhome",
  EmpHome = "emphome",
  JobFullView = "jobfullview",
}

const router = createBrowserRouter([
  {
    path: RoutePaths.Root,
    element: <Home />,
  },
  {
    path: RoutePaths.DevHome,
    element: <DeveloperHome />,
  },
  {
    path: RoutePaths.JobFullView,
    element: <JobFullView />,
  },
  {
    path: RoutePaths.EmpHome,
    element: <EmployerHome />,
  },
]);

function App() {
  return <RouterProvider router={router} />;
}

export default App;
