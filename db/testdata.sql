CREATE TABLE books (
   id integer GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
   title varchar(40) NOT NULL,
   subtitle varchar(40),
   summary text NOT NULL,
   content text NOT NULL,
   authorname varchar(40) NOT NULL,
   coauthor text,
   kindle text,
   paperback text,
   ebook text,
   tag varchar(40)
);

INSERT INTO books (title, subtitle, summary, content, authorname, coauthor, kindle, paperback, ebook, tag) VALUES   
(
   'THE KCNA BOOK',
   'Kubernetes Cloud Native Associate',
   'Kubernetes and cloud native technologies are reshaping the world. The KCNA certification lets you prove your knowledge and opens you up to the best jobs, best projects, and best companies. The KCNA Book covers everything needed to pass the exam, with over 250 sample questions and a full sample test.',
   'Building apps as small',
   'Nigel Poulton',
   NULL,
   'https://www.amazon.com/KCNA-Book-Kubernetes-Native-Associate-ebook/dp/B09SQ4R2TM/ref=tmm_kin_swatch_0?_encoding=UTF8&qid=1655195717&sr=8-1',
   'https://www.amazon.com/KCNA-Book-Kubernetes-Native-Associate/dp/B09SNKVG5M/ref=tmm_pap_swatch_0?_encoding=UTF8&qid=1655195717&sr=8-1',
   'https://leanpub.com/thekcnabook',
   'Nigels latest book'
);
