UPDATE sessions
SET
    handle = $2,
    access_jwt = $3,
    refresh_jwt = $4,
    updated_at = NOW()
    WHERE did = $1;