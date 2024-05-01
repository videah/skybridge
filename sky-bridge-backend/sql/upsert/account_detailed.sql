INSERT INTO accounts (id, did, banner, avatar, followers_count, follows_count, posts_count, description)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
ON CONFLICT (did)
    DO UPDATE SET banner          = $3,
                  avatar          = $4,
                  followers_count = $5,
                  follows_count   = $6,
                  posts_count     = $7,
                  description     = $8,
                  updated_at      = NOW()
RETURNING *;