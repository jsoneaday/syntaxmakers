import { render, screen } from "@testing-library/react";
import "@testing-library/jest-dom/extend-expect";
import Home from "./Home";
import { RouterProvider, createBrowserRouter } from "react-router-dom";

const router = createBrowserRouter([
  {
    path: "/",
    element: <Home />,
  },
]);

function WrappedHome() {
  return <RouterProvider router={router} />;
}

describe("Test Home", () => {
  it("check snapshot", async () => {
    render(<WrappedHome />);
    const home = screen.getByTestId("home-page");

    expect(home).toBeInTheDocument();
    expect(home).toMatchSnapshot();
  });
});
