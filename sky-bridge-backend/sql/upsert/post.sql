WITH e AS (
    INSERT INTO posts (id, cid, uri, author_did)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (cid)
            DO NOTHING
        RETURNING *)
SELECT *
FROM e
UNION
SELECT *
FROM posts
WHERE cid = $2;