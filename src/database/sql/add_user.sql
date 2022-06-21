INSERT INTO users (tid,uname) 
VALUES ($1,$2)
RETURNING $table_fields;