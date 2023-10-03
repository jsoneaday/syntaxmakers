import configureStore from "redux-mock-store";
import { defaultDevProfile, setupModalOnRoot } from "../__test__/Fixtures";
import { Provider as ReduxProvider } from "react-redux";
import { render, screen } from "@testing-library/react";
import Modal from "./Modal";

describe("Test Modal", () => {
  beforeEach(() => {
    setupModalOnRoot();
  });

  it("loads on render", async () => {
    const mockStore = configureStore();
    const store = mockStore({
      profile: defaultDevProfile,
    });

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
