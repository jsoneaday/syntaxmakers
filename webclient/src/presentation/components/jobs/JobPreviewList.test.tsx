import configureStore from "redux-mock-store";
import { defaultEmpProfile } from "../../__test__/Fixtures";
import { UiDevOrEmployer } from "../../models/DevOrEmployer";
import { render, screen } from "@testing-library/react";
import * as DevOrEmployerHooks from "../../common/redux/devOrEmployer/DevOrEmployerHooks";
import { Provider } from "react-redux";
import { RouterProvider, createBrowserRouter } from "react-router-dom";
import JobPost from "../../models/JobPost";
/// @ts-ignore
import { v4 as uuidv4 } from "uuid";
import JobPreviewList from "./JobPreviewList";
import { EMP_ROUTE_PREFIX } from "../../../App";

const title1 = "Senior Super Dev";
const title2 = "French Frie Dev";

const jobPosts: JobPost[] = [];
jobPosts.push({
  key: uuidv4(),
  id: 1,
  updatedAt: "2022/01/03T22:21:02",
  title: title1,
  description: '[{"type":"paragraph","children":[{"text":"Testing A"}]}]',
  employerId: 1,
  employerName: "Employer John",
  companyId: 1,
  companyName: "Company A",
  isRemote: true,
  primaryLangId: 1,
  primaryLangName: "JavaScript",
  secondaryLangId: 1,
  secondaryLangName: "C#",
  industryId: 1,
  industryName: "Finance",
  salaryId: 1,
  salary: "200000",
  countryName: undefined,
});
jobPosts.push({
  key: uuidv4(),
  id: 2,
  updatedAt: "2022/01/03T22:21:02",
  title: title2,
  description: '[{"type":"paragraph","children":[{"text":"Testing A"}]}]',
  employerId: 2,
  employerName: "Employer Linda",
  companyId: 2,
  companyName: "Company B",
  isRemote: false,
  primaryLangId: 2,
  primaryLangName: "Ruby",
  secondaryLangId: 2,
  secondaryLangName: "Python",
  industryId: 2,
  industryName: "Finance",
  salaryId: 2,
  salary: "200000",
  countryName: "United States",
});

const router = createBrowserRouter([
  {
    path: "/",
    element: <JobPreviewList jobPosts={jobPosts} />,
  },
]);

describe("Test JobPreviewList", () => {
  beforeAll(() => {
    jest
      .spyOn(DevOrEmployerHooks, "useDevOrEmployer")
      .mockImplementation(() => {
        return [UiDevOrEmployer.Employer, (_devOrEmp: UiDevOrEmployer) => {}];
      });
  });

  it("matches snapshot", async () => {
    const mockStore = configureStore();
    const store = mockStore({
      profile: defaultEmpProfile,
      devOrEmployer: UiDevOrEmployer.Employer,
    });

    const result = render(
      <Provider store={store}>
        <RouterProvider router={router} />
      </Provider>
    );

    expect(result).toMatchSnapshot();
  });

  it("links go to emphome", async () => {
    const mockStore = configureStore();
    const store = mockStore({
      profile: defaultEmpProfile,
      devOrEmployer: UiDevOrEmployer.Employer,
    });

    render(
      <Provider store={store}>
        <RouterProvider router={router} />
      </Provider>
    );

    expect(screen.getByText(title1).closest("a")).toHaveAttribute(
      "href",
      `/${EMP_ROUTE_PREFIX}/jobfullview`
    );
  });
});
