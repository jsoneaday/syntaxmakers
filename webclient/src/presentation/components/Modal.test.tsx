import configureStore from "redux-mock-store";
import { defaultDevProfile } from "../__test__/Fixtures";
import { Provider as ReduxProvider } from "react-redux";
import { render, screen } from "@testing-library/react";
import Modal from "./Modal";
const reactmodal = require("react-modal");

describe("Test Modal", () => {
  beforeEach(() => {
    document.body.innerHTML = `
      <div id="root">
      </div>
    `;
  });

  it("loads on render", async () => {
    const mockStore = configureStore();
    const store = mockStore({
      profile: defaultDevProfile,
    });
    reactmodal.setAppElement("#root");

    render(
      <ReduxProvider store={store}>
        <Modal isOpen={true} toggleOpen={() => {}}>
          <div>hello world</div>
        </Modal>
      </ReduxProvider>
    );

    const result = screen.getByText("hello world");
    expect(result).not.toBeFalsy();
  });
});
