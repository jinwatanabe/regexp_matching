import { User } from "@/domain/User";

export class UserInputPort {
  constructor() {}

  async getUsers(): Promise<User[]> {
    throw new Error("Not implemented");
  }
}
