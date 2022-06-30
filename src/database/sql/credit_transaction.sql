INSERT INTO credit (credit_amount,reason,uid,transaction_date) 
VALUES ($1,$2,$3,$4)
RETURNING $table_fields;