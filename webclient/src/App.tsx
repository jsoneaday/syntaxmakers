import { createBrowserRouter, RouterProvider } from "react-router-dom";
import Home from "./presentation/pages/Home";
import JobFullviewPage from "./presentation/pages/developer/JobFullviewPage";
import EmpJobPosts from "./presentation/pages/employer/EmpJobPosts";
import EmpJobApplicants from "./presentation/pages/employer/EmpJobApplicants";
import DevJobPosts from "./presentation/pages/developer/DevJobPosts";
import DevSavedJobs from "./presentation/pages/developer/DevSavedJobs";
import DevJobAlerts from "./presentation/pages/developer/DevJobAlerts";
import EmpPostNewJob from "./presentation/pages/employer/EmpPostNewJob";

export const DEV_ROUTE_PREFIX = "devhome";
export const EMP_ROUTE_PREFIX = "emphome";

export enum RoutePaths {
  Root = "/",
  DevJobPosts = `/${DEV_ROUTE_PREFIX}/jobposts`,
  DevSavedJobs = `/${DEV_ROUTE_PREFIX}/savedjobs`,
  DevJobAlerts = `/${DEV_ROUTE_PREFIX}/jobalerts`,
  EmpPostNewJob = `/${EMP_ROUTE_PREFIX}/postnewjob`,
  EmpJobPosts = `/${EMP_ROUTE_PREFIX}/jobposts`,
  EmpJobApplicants = `/${EMP_ROUTE_PREFIX}/jobapplicants`,
  JobFullView = `/jobfullview`,
}

const router = createBrowserRouter([
  {
    path: RoutePaths.Root,
    element: <Home />,
  },
  {
    path: RoutePaths.DevJobPosts,
    element: <DevJobPosts />,
  },
  {
    path: RoutePaths.DevSavedJobs,
    element: <DevSavedJobs />,
  },
  {
    path: RoutePaths.DevJobAlerts,
    element: <DevJobAlerts />,
  },
  {
    path: RoutePaths.EmpPostNewJob,
    element: <EmpPostNewJob />,
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
