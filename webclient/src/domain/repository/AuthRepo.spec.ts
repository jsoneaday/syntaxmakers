import { DevOrEmployer, login } from "./AuthRepo";

describe("Test AuthRepo", () => {
  it("confirm login returns true if sent valid params", async () => {
    const result = await login(
      DevOrEmployer.Developer,
      "test@test.com",
      "test123"
    );

    expect(result).toBe(true);
  });
});
