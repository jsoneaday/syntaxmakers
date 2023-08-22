import { createBrowserRouter, RouterProvider } from "react-router-dom";
import Home from "./presentation/pages/Home";
import DeveloperHome from "./presentation/pages/developer/DeveloperHome";
import JobFullviewPage from "./presentation/pages/developer/JobFullviewPage";
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
    element: <JobFullviewPage />,
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
