import { render, waitFor, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import "@testing-library/jest-dom/extend-expect";
import App from "./App";
import { Provider as ReduxProvider } from "react-redux";
import * as DevRepo from "./domain/repository/DeveloperRepo";
import * as JobRepo from "./domain/repository/JobRepo";
import { Developer } from "./domain/repository/DeveloperRepo";
import { faker } from "@faker-js/faker";
import { getFakeFullName } from "./domain/TestUtils";
import configureStore from "redux-mock-store";
import DevProfile from "./presentation/models/DevProfile";
/// @ts-ignore
import { v4 as uuidv4 } from "uuid";

const defaultDevProfile: DevProfile = {
  key: uuidv4(),
  id: "1",
  updatedAt: new Date().toISOString(),
  userName: faker.internet.userName(),
  fullName: getFakeFullName(),
  email: faker.internet.email(),
  primaryLangId: "1",
};
const mockStore = configureStore();
const store = mockStore({
  profile: defaultDevProfile,
});

describe("Test Home page", () => {
  beforeEach(() => {
    jest.spyOn(DevRepo, "getDeveloper").mockImplementation(
      () =>
        new Promise((res) => {
          res(
            new Developer(
              "1",
              new Date().toISOString(),
              faker.internet.userName(),
              getFakeFullName(),
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
              "200000"
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
