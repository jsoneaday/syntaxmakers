import { render } from "@testing-library/react";
import App from "./App";
/// @ts-ignore
import { v4 as uuidv4 } from "uuid";

describe("Test Home page", () => {
  it("loads App component with home route as default route", async () => {
    const { getByTestId } = render(<App />);

    const home = getByTestId("home-page");
    expect(home).toBeInTheDocument();
  });
});
