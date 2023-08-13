import { render, waitFor, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import "@testing-library/jest-dom/extend-expect";
import App from "./App";
import { Provider as ReduxProvider } from "react-redux";
import { store } from "./presentation/common/redux/Store";
import * as DevRepo from "./domain/repository/DeveloperRepo";
import * as JobRepo from "./domain/repository/JobRepo";
import { Developer } from "./domain/repository/DeveloperRepo";
import { faker } from "@faker-js/faker";
import { getFakeFullName } from "./domain/TestUtils";

describe("Test Home page", () => {
  beforeEach(() => {
    jest.spyOn(DevRepo, "getDeveloper").mockImplementation(
      () =>
        new Promise((res) => {
          res(
            new Developer(
              "1",
              new Date(),
              "dave",
              "David Choi",
              "test@test.com",
              "1",
              undefined
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
              getFakeFullName(),
              "1",
              faker.company.name(),
              faker.lorem.sentence(50),
              faker.lorem.sentence(100),
              true,
              "1",
              faker.animal.dog(),
              "2",
              faker.animal.bird(),
              "1",
              faker.commerce.product(),
              "1",
              "200000",
              undefined, // normal format for file data received over wire
              undefined,
              undefined
            ),
          ]);
        })
    );
  });

  it("loads with home route by default routes", async () => {
    const { getByTestId } = render(<App />);

    const home = getByTestId("home-page");
    expect(home).toBeInTheDocument();
  });

  it("load home page then switch to developer", async () => {
    render(
      <ReduxProvider store={store}>
        <App />
      </ReduxProvider>
    );

    const devLink = screen.getByTestId("dev-link");
    await userEvent.click(devLink);

    const devPage = await waitFor(() => screen.getByTestId("developer-page"));
    expect(devPage).toBeInTheDocument();
  });
});
