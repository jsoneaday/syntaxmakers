import { render } from "@testing-library/react";
import "@testing-library/jest-dom/extend-expect";
import App from "./App";

describe("Test Home page", () => {
  it("loads with home and developer routes", async () => {
    const { getByTestId } = render(<App />);

    const home = getByTestId("home-page");
    expect(home).toBeInTheDocument();
  });
});
