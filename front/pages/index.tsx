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
import { User } from "@/types/User";

const inter = Inter({ subsets: ["latin"] });
const users: User[] = [
  {
    id: 1,
    name: "山田太郎",
    email: "test1@example.com",
  },
  {
    id: 2,
    name: "山田花子",
    email: "test2@example.com",
  },
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
                <Tr key={user.id}>
                  <Td>{user.id}</Td>
                  <Td>{user.name}</Td>
                  <Td>{user.email}</Td>
                </Tr>
              ))}
            </Tbody>
          </Table>
        </TableContainer>
      </main>
    </>
  );
}
