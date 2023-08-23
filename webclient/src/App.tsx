import { createBrowserRouter, RouterProvider } from "react-router-dom";
import Home from "./presentation/pages/Home";
import DeveloperHome from "./presentation/pages/developer/DeveloperHome";
import JobFullviewPage from "./presentation/pages/developer/JobFullviewPage";
import EmpJobPosts from "./presentation/pages/employer/EmpJobPosts";
import EmpJobApplicants from "./presentation/pages/employer/EmpJobApplicants";

export enum RoutePaths {
  Root = "/",
  DevHome = "/devhome",
  EmpJobPosts = "/emphome/jobposts",
  EmpJobApplicants = "/emphome/jobapplicants",
  JobFullView = "/jobfullview",
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
    path: RoutePaths.EmpJobPosts,
    element: <EmpJobPosts />,
  },
  {
    path: RoutePaths.EmpJobApplicants,
    element: <EmpJobApplicants />,
  },
  {
    path: RoutePaths.JobFullView,
    element: <JobFullviewPage />,
  },
]);

function App() {
  return <RouterProvider router={router} />;
}

export default App;
