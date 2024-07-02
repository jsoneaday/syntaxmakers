import { createBrowserRouter, RouterProvider } from "react-router-dom";
import Home from "./presentation/pages/Home";
import EmpJobPosts from "./presentation/pages/employer/EmpJobPosts";
import EmpJobApplicants from "./presentation/pages/employer/EmpJobApplicants";
import DevJobSearch from "./presentation/pages/developer/DevJobSearch";
import DevAppliedJobs from "./presentation/pages/developer/DevAppliedJobs";
import DevJobFullview from "./presentation/pages/developer/DevJobFullview";
import EmpJobFullview from "./presentation/pages/employer/EmpJobFullview";
import DevProfile from "./presentation/pages/developer/DevProfile";

export const DEV_ROUTE_PREFIX = "devhome";
export const EMP_ROUTE_PREFIX = "emphome";

export enum RoutePaths {
  Root = "/",
  DevJobSearch = `/${DEV_ROUTE_PREFIX}/searchjobs`,
  DevAppliedJobs = `/${DEV_ROUTE_PREFIX}/appliedjobs`,
  DevProfile = `/${DEV_ROUTE_PREFIX}/profile`,
  DevJobFullView = `/${DEV_ROUTE_PREFIX}/jobfullview`,
  EmpPostNewJob = `/${EMP_ROUTE_PREFIX}/postnewjob`,
  EmpJobPosts = `/${EMP_ROUTE_PREFIX}/jobposts`,
  EmpJobApplicants = `/${EMP_ROUTE_PREFIX}/jobapplicants`,
  EmpJobFullView = `/${EMP_ROUTE_PREFIX}/jobfullview`,
}

const router = createBrowserRouter([
  {
    path: RoutePaths.Root,
    element: <Home />,
  },
  {
    path: RoutePaths.DevJobSearch,
    element: <DevJobSearch />,
  },
  {
    path: `${RoutePaths.DevJobSearch}/:search`,
    element: <DevJobSearch />,
  },
  {
    path: RoutePaths.DevAppliedJobs,
    element: <DevAppliedJobs />,
  },
  {
    path: RoutePaths.DevProfile,
    element: <DevProfile />,
  },
  {
    path: RoutePaths.DevJobFullView,
    element: <DevJobFullview />,
  },
  {
    path: RoutePaths.EmpPostNewJob,
    element: <EmpJobFullview />,
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
    path: RoutePaths.EmpJobFullView,
    element: <EmpJobFullview />,
  },
]);

function App() {
  return <RouterProvider router={router} />;
}

export default App;
