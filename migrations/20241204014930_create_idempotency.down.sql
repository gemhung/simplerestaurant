-- Add down migration script here
DO
$do$
BEGIN 
   FOR i IN 1..100 LOOP
      -- Create the ordered_items table within the schema
      EXECUTE format('
            DROP TABLE table%s.idempotency
      ', i);
   END LOOP;
END
$do$;

DROP TYPE  header_pair;
