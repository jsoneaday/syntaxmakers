import { faker } from "@faker-js/faker";
import { Provider as ReduxProvider } from "react-redux";
import * as DevRepo from "../../../domain/repository/DeveloperRepo";
import * as JobRepo from "../../../domain/repository/JobRepo";
import { Developer } from "../../../domain/repository/DeveloperRepo";
import { render, waitFor, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import "@testing-library/jest-dom/extend-expect";
import configureStore from "redux-mock-store";
import { defaultDevProfile } from "../../__test__/Fixtures";
import App from "../../../App";

describe("Test Developer page", () => {
  beforeAll(() => {
    jest.spyOn(DevRepo, "getDeveloper").mockImplementation(
      () =>
        new Promise((res) => {
          res(
            new Developer(
              "1",
              new Date().toISOString(),
              "testuser",
              "Tester Test",
              faker.internet.email(),
              "1"
            )
          );
        })
    );

    jest.spyOn(JobRepo, "getJobsByDevProfile").mockImplementation(
      () =>
        new Promise((res) => {
          res([
            new JobRepo.Job(
              "1",
              new Date().toISOString(),
              "1",
              "Employer Tester",
              "1",
              "Company A",
              "Senior React Developer",
              "This role is front end focused. Must have at least 5 years of React and TypeScript",
              true,
              "1",
              "Rust",
              "2",
              "Go",
              "1",
              "United States",
              "1",
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
