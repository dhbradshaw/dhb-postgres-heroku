# dhb-postgres-heroku
Given a DATABASE_URL, it should be dead simple to connect to a Heroku postgres database.

This crate makes it dead simple with rust:

You pass a DATABASE_URL to the postgres_client function and get a working client back, as in
```rust
let mut client = get_client(&database_url);
```

The reason I found the work to create this crate necessary is that connecting to Heroku has 2 quirks.
1. On the one hand, it requires that we have a secure connection.
2. On the other hand, it uses self-verified certificates.  So we have to enable ssl, but turn off verification. 
