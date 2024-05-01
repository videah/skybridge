INSERT INTO sessions
    ( did, handle, access_jwt, refresh_jwt )
VALUES
    ( $1, $2, $3, $4 )
RETURNING *