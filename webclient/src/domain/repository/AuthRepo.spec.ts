import { DevOrEmployer, login } from "./AuthRepo";

describe("Test AuthRepo", () => {
  it("confirm login returns true if sent valid params", async () => {
    const result = await login(
      DevOrEmployer.Developer,
      "jon@jon.com",
      "test123"
    );

    console.log("login id", result.toString());
    expect(result.toString()).toBe("1");
  });
});
