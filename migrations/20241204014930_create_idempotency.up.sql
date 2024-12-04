-- Add up migration script here
CREATE TYPE header_pair AS (
    name TEXT,
    value BYTEA
);

DO
$do$
BEGIN 
   FOR i IN 1..100 LOOP
      EXECUTE format('
            CREATE TABLE table%s.idempotency (
                idempotency_key TEXT NOT NULL,
                response_status_code SMALLINT NOT NULL,
                response_headers header_pair[] NOT NULL,
                response_body BYTEA NOT NULL,
                created_at timestamptz NOT NULL,
                PRIMARY KEY(idempotency_key)
            );
      ', i);
   END LOOP;
END
$do$;
