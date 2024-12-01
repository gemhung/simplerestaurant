-- Add up migration script here
DO
$do$
BEGIN 
   FOR i IN 1..100 LOOP
      EXECUTE format('CREATE SCHEMA table%s;', i);
   END LOOP;
END
$do$;
