import Container from 'react-bootstrap/Container';
import Nav from 'react-bootstrap/Nav';
import Navbar from 'react-bootstrap/Navbar';

function NavBar() {
    return (
        <Navbar bg="dark" data-bs-theme="dark">
          <Container>
            <Navbar.Brand href="#home">Algorithms Visualization</Navbar.Brand>
            <Nav className="me-auto">
              <Nav.Link href="#home">Home</Nav.Link>
              <Nav.Link href="#sorting">Sorting</Nav.Link>
              <Nav.Link href="#pathfiding">Pathfiding</Nav.Link>
            </Nav>
          </Container>
        </Navbar>
    );
  }
  
  export default NavBar;