## 1. Create database
```
sudo su postgres
psql
```
```
CREATE DATABASE <database> WITH OWNER <user_name>;
\c <database>
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
```

## 2. Create config
```
cp .env.example .env
cp Rocket.toml.example Rocket.toml
```
Then change the database url in both files to:
```
postgres://<user>:<password>@localhost/<database>
```

