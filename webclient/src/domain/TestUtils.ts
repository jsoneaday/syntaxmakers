import { faker } from "@faker-js/faker";

export function getFakeFullName() {
  return faker.person.fullName();
}
