# RUST BLOG

Basic API REST made in rust with actix and diesel. Handling errors and JWT auth.

The URL to the public API is https://rust-api-blog.herokuapp.com

# Routes

## Users

**`GET`** `/users/`
Get all the users

**Response**

```json
[
  {
    "id": "716ab337-66ef-4319-bf4d-2ba666b6d81f",
    "username": "Learnst",
    "email": "correo@corre.com",
    "password": "$2b$10$D7UiUfuyl6LRuAjHkZNsn.2Ks5ZtP0dING7O2ICUDOr03iJA2BtSG",
    "is_admin": true,
    "created_at": "2022-06-09T22:19:13.947689",
    "updated_at": "2022-06-09T22:19:13.947689"
  }
]
```

**`POST`** `/users/`
Register new user

**Request**

```json
{
  "username": "Learnst",
  "email": "correo@corre.com",
  "password": "12345",
  "is_admin": true
}
```

**Response**

```json
{
  "id": "716ab337-66ef-4319-bf4d-2ba666b6d81f",
  "username": "Learnst",
  "email": "correo@corre.com",
  "password": "$2b$10$D7UiUfuyl6LRuAjHkZNsn.2Ks5ZtP0dING7O2ICUDOr03iJA2BtSG",
  "is_admin": true,
  "created_at": "2022-06-09T22:19:13.947689",
  "updated_at": "2022-06-09T22:19:13.947689"
}
```

**`POST`** `/users/login`
Login.

**Request**

```json
{
  "username": "Learnst",
  "password": "12345"
}
```

**Response**

```json
{
  "jwt": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpZCI6IjE0YmFkMTZkLTJlOWEtNDNlMi04NWU4LTBiMWVlMDI2ZjNiMSIsImlhdCI6MTY1NDg3MjY4NywiZXhwIjoxNjU0ODc2Mjg3LCJlbWFpbCI6ImNvcnJlb0Bjb3JyZW8uY29tIn0.JT-Vf5oTrXNAKphjpI9d2uSoM902pQLWdTZG-s-xHh8"
}
```

## Posts

For every endpoint in posts you will need to send the JWT in `Authorization` header.

**`GET`** `/posts/`
Get all the posts.

**Request**
JWT in `Authorization` header.

**Response**

```json
[
  {
    "id": "9f7a04ce-c5dc-491a-839a-a3f4efbbb8bc",
    "title": "New Rust API",
    "slug": "new-rust-api",
    "user_id": "716ab337-66ef-4319-bf4d-2ba666b6d81f",
    "body": "copy ans paste",
    "published": true,
    "created_at": "2022-06-10T14:58:59.872727",
    "updated_at": "2022-06-10T14:58:59.872727"
  }
]
```

**`POST`** `/posts/`
Create new post.

**Request**
JWT in `Authorization` header.

```json
{
  {
    "title": "New Rust API",
    "body": "copy ans paste",
    "published": true
  }
}
```

**Response**

```json
{
  {
    "id": "9f7a04ce-c5dc-491a-839a-a3f4efbbb8bc",
    "title": "New Rust API",
    "slug": "new-rust-api",
    "user_id": "716ab337-66ef-4319-bf4d-2ba666b6d81f",
    "body": "copy ans paste",
    "published": true,
    "created_at": "2022-06-10T14:58:59.872727",
    "updated_at": "2022-06-10T14:58:59.872727"
  }
}
```
