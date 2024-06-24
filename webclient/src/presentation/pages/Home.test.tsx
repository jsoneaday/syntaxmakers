import { render, screen } from "@testing-library/react";
import Home from "./Home";
import { RouterProvider, createBrowserRouter } from "react-router-dom";
import { defaultDevProfile, setupModalOnRoot } from "../__test__/Fixtures";
import configureStore from "redux-mock-store";
import { Provider as ReduxProvider } from "react-redux";

const router = createBrowserRouter([
  {
    path: "/",
    element: <Home />,
  },
]);

function WrappedHome() {
  const mockStore = configureStore();
  const store = mockStore({
    profile: defaultDevProfile,
  });

  return (
    <ReduxProvider store={store}>
      <RouterProvider router={router} />
    </ReduxProvider>
  );
}

describe("Test Home", () => {
  beforeAll(() => {
    setupModalOnRoot();
  });

  it("matches snapshot", async () => {
    render(<WrappedHome />);
    const home = screen.getByTestId("home-page");

    expect(home).toBeInTheDocument();
    expect(home).toMatchSnapshot();
  });
});
