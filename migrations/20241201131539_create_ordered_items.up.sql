-- Add up migration script here
DO
$do$
BEGIN 
   FOR i IN 1..100 LOOP
      -- Create the ordered_items table within the schema
      EXECUTE format('
          CREATE TABLE table%s.ordered_items (
              id SERIAL PRIMARY KEY,                -- Primary key
              name VARCHAR(255) NOT NULL,           -- Name of the item (non-null string)
              status VARCHAR(50) NOT NULL,          -- Status of the item (non-null string)
              cooking_time BIGINT NOT NULL,         -- Cooking time in seconds
              idempotent_token VARCHAR(255) UNIQUE, -- Unique idempotent token
              created_at TIMESTAMP DEFAULT NOW()    -- Timestamp of creation with default value
          );
      ', i);
   END LOOP;
END
$do$;
