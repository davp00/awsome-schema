pub const DEFAULT_SCHEMA_TEMPLATE: &str = r#"datasource db {
  provider = "surrealdb"
  url      = env("SURREALDB_URL")
  namespace = "app"
  database  = "main"
}

generator client {
  provider = "typescript"
  output   = "./generated"
}

model User {
  id        @id
  email     string @unique
  name      string
  age       int?
  profile   object?
  tags      string[]
  createdAt datetime @value(time::now()) @readonly
  updatedAt datetime @updated(time::now())

  posts     Post[] @relation("Likes")

  @@table(schemafull)
  @@permissions("FULL")
}

model Post {
  id        @id
  title     string
  content   string
  author    User @link
  createdAt datetime @value(time::now()) @readonly
  updatedAt datetime @updated(time::now())

  @@table(schemafull)
  @@index([title])
}

edge Likes {
  in  User
  out Post

  createdAt datetime @value(time::now()) @readonly
  updatedAt datetime @updated(time::now())

  @@table(schemafull)
}
"#;
