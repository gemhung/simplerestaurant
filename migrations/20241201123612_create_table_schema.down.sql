-- Add down migration script here
DO
$do$
BEGIN 
   FOR i IN 1..100 LOOP
      EXECUTE format('DROP SCHEMA table%s;', i);
   END LOOP;
END
$do$;
