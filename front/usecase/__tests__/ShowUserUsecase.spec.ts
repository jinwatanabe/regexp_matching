import { when } from "jest-when";

import { Email, Name, User, UserId } from "../../domain/User";
import { UserInputPort } from "../port/UserInputPort";
import { UserOutputPort } from "../port/UserOutputPort";
import { ShowUserUsecase } from "../ShowUserUsecase";

describe("UserUsecase", () => {
  it("should be defined", async () => {
    const inputPortMock = {} as UserInputPort;
    const getUsersMock = jest.fn();
    inputPortMock.getUsers = getUsersMock;

    const users: User[] = [
      new User(new UserId("1"), new Name("name1"), new Email("email1")),
      new User(new UserId("2"), new Name("name2"), new Email("email2")),
    ];

    when(getUsersMock).calledWith().mockResolvedValue(users);

    const OutputPortMock = {} as UserOutputPort;
    const displayMock = jest.fn();
    OutputPortMock.display = displayMock;

    const target = new ShowUserUsecase(inputPortMock, OutputPortMock);
    await target.execute();

    expect(getUsersMock).toBeCalledTimes(1);
    expect(displayMock).toBeCalledTimes(1);
    expect(displayMock).toBeCalledWith(users);
  });
});
