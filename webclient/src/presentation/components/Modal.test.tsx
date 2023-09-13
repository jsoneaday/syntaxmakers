import configureStore from "redux-mock-store";
import { defaultDevProfile } from "../__test__/Fixtures";
import { Provider as ReduxProvider } from "react-redux";
import { render, screen } from "@testing-library/react";
import Modal from "./Modal";

describe("Test Modal", () => {
  it("loads on render", async () => {
    const mockStore = configureStore();
    const store = mockStore({
      profile: defaultDevProfile,
    });

    render(
      <div id="root">
        <ReduxProvider store={store}>
          <Modal isOpen={true} toggleOpen={() => {}}>
            <div>hello world</div>
          </Modal>
        </ReduxProvider>
      </div>
    );

    const result = screen.getByText("hello world");
    expect(result).not.toBeFalsy();
  });
});
