INSERT INTO accounts (id, did, avatar)
VALUES ($1, $2, $3)
ON CONFLICT (did)
    DO UPDATE SET avatar     = $3,
                  updated_at = NOW()
RETURNING *;