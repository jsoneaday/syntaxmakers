import { render } from "@testing-library/react";
import App from "./App";
/// @ts-ignore
import { v4 as uuidv4 } from "uuid";
import configureStore from "redux-mock-store";
import { defaultDevProfile } from "./presentation/__test__/Fixtures";
import { Provider as ReduxProvider } from "react-redux";

describe("Test Home page", () => {
  it("loads App component with home route as default route", async () => {
    const mockStore = configureStore();
    const store = mockStore({
      profile: defaultDevProfile,
    });
    const { getByTestId } = render(
      <ReduxProvider store={store}>
        <App />
      </ReduxProvider>
    );

    const home = getByTestId("home-page");
    expect(home).toBeInTheDocument();
  });
});
