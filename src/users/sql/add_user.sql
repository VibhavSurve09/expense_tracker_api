INSERT INTO users (uname,password_) 
VALUES ($1,$2)
RETURNING $table_fields;