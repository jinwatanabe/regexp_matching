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

const inter = Inter({ subsets: ["latin"] });

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
              <Tr>
                <Td>1</Td>
                <Td>山田太郎</Td>
                <Td>test1@example.com</Td>
              </Tr>
            </Tbody>
          </Table>
        </TableContainer>
      </main>
    </>
  );
}
