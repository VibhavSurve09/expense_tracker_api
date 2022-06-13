INSERT INTO debit (debit_amount,reason,uid) 
VALUES ($1,$2,$3)
RETURNING $table_fields;