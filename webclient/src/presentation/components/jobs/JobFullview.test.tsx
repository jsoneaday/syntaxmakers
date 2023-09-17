import JobPost from "../../models/JobPost";
/// @ts-ignore
import { v4 as uuidv4 } from "uuid";
import { render, screen } from "@testing-library/react";
import JobFullview from "./JobFullview";
import { RouterProvider, createBrowserRouter } from "react-router-dom";
import { Provider } from "react-redux";
import configureStore from "redux-mock-store";
import { defaultDevProfile } from "../../__test__/Fixtures";
import { DevOrEmployer } from "../../models/DevOrEmployer";

const router = createBrowserRouter([
  {
    path: "/",
    element: <JobFullview readOnly={true} />,
  },
]);

const title = "Senior JavaScript Developer";
const employerName = "John Dole";
const isRemote = true;
const companyName = "Acme Inc";
const primaryLangName = "JavaScript";
const secondaryLangName = "C#";
const industryName = "Finance";
const salary = "200000";

const jobPost: JobPost = {
  key: uuidv4(),
  id: "1",
  updatedAt: "2022/01/03T22:21:02",
  title,
  description: "A description",
  employerId: "1",
  employerName,
  companyId: "1",
  companyName,
  isRemote,
  primaryLangId: "1",
  primaryLangName,
  secondaryLangId: "1",
  secondaryLangName,
  industryId: "1",
  industryName,
  salaryId: "1",
  salary,
};

jest.mock("react-router-dom", () => ({
  ...jest.requireActual("react-router-dom"),
  useLocation: () => {
    return {
      state: jobPost,
      key: "123",
      pathname: "",
      search: "",
      hash: "",
    };
  },
}));

describe("Test JobFullview", () => {
  it("matches snapshot", async () => {
    const mockStore = configureStore();
    const store = mockStore({
      profile: defaultDevProfile,
      devOrEmployer: DevOrEmployer.Developer,
    });

    const result = render(
      <Provider store={store}>
        <RouterProvider router={router} />
      </Provider>
    );

    expect(result).toMatchSnapshot();
  });

  it("has correct title", async () => {
    const mockStore = configureStore();
    const store = mockStore({
      profile: defaultDevProfile,
      devOrEmployer: DevOrEmployer.Developer,
    });

    render(
      <Provider store={store}>
        <RouterProvider router={router} />
      </Provider>
    );

    screen.getByText(title);
    screen.getByText(`Contact ${employerName}`);
    screen.getByText("Remote");
    screen.getByText(companyName);
    screen.getByText(`Primary Language ${primaryLangName}`);
    screen.getByText(`Secondary Language ${secondaryLangName}`);
    screen.getByText(`Industry ${industryName}`);
    screen.getByText(`Base Salary ${salary}`);
  });
});
