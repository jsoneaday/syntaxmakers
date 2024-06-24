import configureStore from "redux-mock-store";
import { defaultEmpProfile } from "../../__test__/Fixtures";
import { DevOrEmployer } from "../../models/DevOrEmployer";
import { render, screen } from "@testing-library/react";
import { Provider } from "react-redux";
import { RouterProvider, createBrowserRouter } from "react-router-dom";
import JobPreview from "./JobPreview";
import JobPost from "../../models/JobPost";
/// @ts-ignore
import { v4 as uuidv4 } from "uuid";
import { faker } from "@faker-js/faker";

const title = faker.string.sample(10);
const employerName = faker.internet.displayName();
const isRemote = true;
const companyName = faker.company.name();
const primaryLangName = "JavaScript";
const secondaryLangName = "C#";
const industryName = "Finance";
const salary = "200000";
const countryName = undefined;

const jobPost: JobPost = {
  key: uuidv4(),
  id: 1,
  updatedAt: "2022/01/03T22:21:02",
  title,
  description: '[{"type":"paragraph","children":[{"text":"Testing Text"}]}]',
  employerId: 1,
  employerName,
  companyId: 1,
  companyName,
  isRemote,
  primaryLangId: 1,
  primaryLangName,
  secondaryLangId: 1,
  secondaryLangName,
  industryId: 1,
  industryName,
  salaryId: 1,
  salary,
  countryName,
};

const router = createBrowserRouter([
  {
    path: "/",
    element: <JobPreview jobPost={jobPost} isSmall={false} />,
  },
]);

describe("Test JobPreview", () => {
  it("matches snapshot", async () => {
    const mockStore = configureStore();
    const store = mockStore({
      profile: defaultEmpProfile,
      devOrEmployer: DevOrEmployer.Employer,
    });

    const result = render(
      <Provider store={store}>
        <RouterProvider router={router} />
      </Provider>
    );

    screen.getByText(title);
    screen.getByText(companyName);
    screen.getByText("Remote");
    screen.getByText(salary);
    expect(result).toMatchSnapshot();
  });
});
