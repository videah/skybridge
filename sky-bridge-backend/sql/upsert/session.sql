INSERT INTO sessions
    ( did, handle, access_jwt, refresh_jwt )
VALUES
    ( $1, $2, $3, $4 )
ON CONFLICT (did)
    DO UPDATE SET
          handle = $2,
          access_jwt = $3,
          refresh_jwt = $4,
          updated_at = NOW()
RETURNING *;