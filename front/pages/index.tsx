import { Inter } from "@next/font/google";
import {
  Button,
  Table,
  TableContainer,
  Tbody,
  Td,
  Th,
  Thead,
  Tr,
} from "@chakra-ui/react";
import { Email, Name, User, UserId } from "@/domain/User";

const inter = Inter({ subsets: ["latin"] });
const users: User[] = [
  new User(
    new UserId("1"),
    new Name("山田太郎"),
    new Email("test1@example.com")
  ),
  new User(
    new UserId("2"),
    new Name("山田花子"),
    new Email("test2@example.com")
  ),
];

export default function Home() {
  return (
    <>
      <main>
        <div className="hello">Hello World</div>
        <Button colorScheme="blue">ChakuraButton</Button>
        <TableContainer className="user-table">
          <Table variant="simple">
            <Thead>
              <Tr>
                <Th>ID</Th>
                <Th>名前</Th>
                <Th>メールアドレス</Th>
              </Tr>
            </Thead>
            <Tbody>
              {users.map((user) => (
                <Tr key={user.id.value}>
                  <Td>{user.id.value}</Td>
                  <Td>{user.name.value}</Td>
                  <Td>{user.email.value}</Td>
                </Tr>
              ))}
            </Tbody>
          </Table>
        </TableContainer>
      </main>
    </>
  );
}
