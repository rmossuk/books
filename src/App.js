import React, { useState, useEffect } from 'react';
import './App.css';
import AppBar from '@mui/material/AppBar';
import Box from '@mui/material/Box';
import Toolbar from '@mui/material/Toolbar';
import Grid from '@mui/material/Grid';
import Container from '@mui/material/Container';
import Link from '@mui/material/Link';
import { maxWidth } from '@mui/system';

function App() {

  const [books, setBooks] = useState([]);
  const [loading, setLoading] = useState(true);

  const fetchData = () => {
    fetch("http://127.0.0.1:3000/api/")
    .then((response) => response.json())
    .then((response) => {
      setBooks(response);
      setLoading(false);
    })
    .catch(() => {
    });
  }

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
          <img
          src="https://i0.wp.com/nigelpoulton.com/wp-content/uploads/2022/11/nigelpoulton_logo_22_colour.png?fit=300%2C60&ssl=1"
          alt=""
          loading="lazy"
        />
         
        </Toolbar>
      </AppBar>
    </Box>

    <div class="main">
      {books.map((book, index) => (
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
                  <Link href={book.audible}>Audible</Link> 
                  <Link href={book.paperback}>Paperback</Link> 
                  <Link href={book.kindle}>Kindle</Link> 
                  <Link href={book.ebook}>ebook</Link>
                </p>
              </div>
              </Grid>
              <Grid item xs={4}>
              <img src={book.image}/>
              </Grid>
            </Grid>
          </Container>
        </div>
      ))}
      </div>
  </div>

  );
}

export default App;
