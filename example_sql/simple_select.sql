-- Simple select statement
SELECT id, name, created_at
FROM users
WHERE active = TRUE
ORDER BY created_at DESC;
