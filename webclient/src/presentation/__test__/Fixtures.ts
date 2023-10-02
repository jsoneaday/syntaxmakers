import DevProfile from "../models/DevProfile";
/// @ts-ignore
import { v4 as uuidv4 } from "uuid";
import EmpProfile from "../models/EmpProfile";
const reactmodal = require("react-modal");

export const userName = "tester";
export const fullName = "Tester Test";
export const email = "test@test.com";
export const updatedAt = "2022-12-03T22:21:02.145Z";

export const defaultDevProfile: DevProfile = {
  key: uuidv4(),
  id: "1",
  updatedAt,
  userName,
  fullName,
  email,
  primaryLangId: "1",
};

export const defaultEmpProfile: EmpProfile = {
  key: uuidv4(),
  id: "1",
  updatedAt,
  userName,
  fullName,
  email,
  companyId: "1",
};

export function setupModalOnRoot() {
  document.body.innerHTML = `
      <div id="root">
      </div>
    `;

  reactmodal.setAppElement("#root");
}
