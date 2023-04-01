import { User } from "@/domain/User";

export class UserOutputPort {
  constructor() {}

  display(users: User[]) {
    throw new Error("Not implemented");
  }
}
