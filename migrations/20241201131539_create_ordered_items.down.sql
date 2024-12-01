-- Add down migration script here
DO
$do$
BEGIN 
   FOR i IN 1..100 LOOP
      EXECUTE format('
          DROP TABLE table%s.ordered_items;
      ', i);
   END LOOP;
END
$do$;
