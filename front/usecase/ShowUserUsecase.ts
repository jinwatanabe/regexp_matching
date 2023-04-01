import { UserInputPort } from "./port/UserInputPort";
import { UserOutputPort } from "./port/UserOutputPort";

export class ShowUserUsecase {
  constructor(
    private inputPort: UserInputPort,
    private outputPort: UserOutputPort
  ) {}

  async execute(): Promise<void> {
    const users = await this.inputPort.getUsers();
    this.outputPort.display(users);
  }
}
