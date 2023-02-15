import React, { useState, useEffect } from 'react';
import './App.css';
import AppBar from '@mui/material/AppBar';
import Box from '@mui/material/Box';
import Toolbar from '@mui/material/Toolbar';
import Grid from '@mui/material/Grid';
import Container from '@mui/material/Container';
import Link from '@mui/material/Link';

function Book({ book, index }) {
  return (
    <div className={"booksContainer"+index}>
      <Container maxWidth="md" className="booksInner">
        <Grid container spacing={2}>
          <Grid item xs={8}>
          <div className="bookHeader">
            <h2>{book.title}</h2>
          </div>
          <div className="bookContent">
            <p>{book.subtitle}</p>
            <p>{book.summary}</p>
            <p>{book.content}</p>
          </div>
          <div className="bookFooter">
            <p>
              <Link href={book.audible} target="_blank">Audible</Link> 
              <Link href={book.paperback} target="_blank">Paperback</Link> 
              <Link href={book.kindle} target="_blank">Kindle</Link> 
              <Link href={book.ebook} target="_blank">ebook</Link>
            </p>
          </div>
          </Grid>
          <Grid item xs={4}>
          <img src={book.image}/>
          </Grid>
        </Grid>
      </Container>
    </div>
  );
}

function App() {

  const [books, setBooks] = useState([]);
  const [loading, setLoading] = useState(true);

  const fetchData = async () => {
    try {
      const response = await fetch("http://127.0.0.1:3000/api/");
      const data = await response.json();
      setBooks(data);
      setLoading(false);
    } catch (error) {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchData()
  }, [])


  if (loading) {
    return <div>Loading...</div>;
  }

  return (
    <div className="App">
      <Box sx={{ flexGrow: 1 }}>
      <AppBar component="nav" className='nigelHeader' >
        <Toolbar>
          <img src="https://i0.wp.com/nigelpoulton.com/wp-content/uploads/2022/11/nigelpoulton_logo_22_colour.png?fit=300%2C60&ssl=1"/> 
        </Toolbar>
      </AppBar>
    </Box>

    <div class="main">
      {books.map((book, index) => (
        <Book key={index} index={index} book={book} />
      ))}
      </div>
  </div>
  );
}

export default App;
