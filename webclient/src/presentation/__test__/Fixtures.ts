import DevProfile from "../models/DevProfile";
/// @ts-ignore
import { v4 as uuidv4 } from "uuid";
import { faker } from "@faker-js/faker";
import { getFakeFullName } from "../../domain/TestUtils";

export const defaultDevProfile: DevProfile = {
  key: uuidv4(),
  id: "1",
  updatedAt: new Date().toISOString(),
  userName: "ronaldo", //faker.internet.userName(),
  fullName: getFakeFullName(),
  email: faker.internet.email(),
  primaryLangId: "1",
};
