import { Provider as ReduxProvider } from "react-redux";
import * as DevRepo from "../../../domain/repository/DeveloperRepo";
import * as JobRepo from "../../../domain/repository/JobRepo";
import { Developer } from "../../../domain/repository/DeveloperRepo";
import { render, waitFor, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import configureStore from "redux-mock-store";
import { defaultDevProfile, setupModalOnRoot } from "../../__test__/Fixtures";
import App from "../../../App";

const job1UpdatedAt = "2023-07-03T22:21:02.145Z";

describe("Test Developer page", () => {
  beforeAll(() => {
    setupModalOnRoot();

    jest.spyOn(DevRepo, "getDeveloper").mockImplementation(
      () =>
        new Promise((res) => {
          res(
            new Developer(
              defaultDevProfile.id,
              defaultDevProfile.updatedAt,
              defaultDevProfile.userName,
              defaultDevProfile.fullName,
              defaultDevProfile.email,
              defaultDevProfile.primaryLangId
            )
          );
        })
    );

    jest.spyOn(JobRepo, "getJobsByDeveloper").mockImplementation(
      () =>
        new Promise((res) => {
          res([
            new JobRepo.Job(
              1,
              job1UpdatedAt,
              1,
              "Employer Tester",
              1,
              "Company A",
              "Senior React Developer",
              "This role is front end focused. Must have at least 5 years of React and TypeScript",
              true,
              1,
              "Rust",
              2,
              "Go",
              1,
              "United States",
              1,
              "200000"
            ),
          ]);
        })
    );
  });

  it("matches snapshot", async () => {
    const mockStore = configureStore();
    const store = mockStore({
      profile: defaultDevProfile,
    });

    render(
      <ReduxProvider store={store}>
        <App />
      </ReduxProvider>
    );

    await userEvent.click(screen.getByTestId("dev-link"));

    let developer = await waitFor(() => screen.getByTestId("developer-page"));
    expect(developer).toMatchSnapshot();
  });
});
