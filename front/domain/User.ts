export class User {
  constructor(
    public readonly id: UserId,
    public readonly name: Name,
    public readonly email: Email
  ) {}
}

export class UserId {
  constructor(public readonly value: string) {}
}

export class Name {
  constructor(public readonly value: string) {}
}

export class Email {
  constructor(public readonly value: string) {}
}
