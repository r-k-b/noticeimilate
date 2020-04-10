#!/bin/bash
set -e

gatekeeper_db_user_pw=$(</run/secrets/gatekeeper_db_user_pw)

psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" <<-EOSQL
    CREATE USER gatekeeper WITH
      LOGIN
      NOSUPERUSER
      NOCREATEDB
      NOCREATEROLE
      INHERIT
      NOREPLICATION
      CONNECTION LIMIT -1
      PASSWORD '${gatekeeper_db_user_pw}';
    
    CREATE USER postgres_exporter PASSWORD 'password';
    ALTER USER postgres_exporter SET SEARCH_PATH TO postgres_exporter,pg_catalog;
    
    -- If deploying as non-superuser (for example in AWS RDS), uncomment the GRANT
    -- line below and replace <MASTER_USER> with your root user.
    -- GRANT postgres_exporter TO <MASTER_USER>
    CREATE SCHEMA postgres_exporter AUTHORIZATION postgres_exporter;
    
    CREATE VIEW postgres_exporter.pg_stat_activity
    AS
    SELECT *
    from pg_catalog.pg_stat_activity;
    
    GRANT SELECT ON postgres_exporter.pg_stat_activity TO postgres_exporter;
    
    CREATE VIEW postgres_exporter.pg_stat_replication AS
    SELECT *
    from pg_catalog.pg_stat_replication;
    
    GRANT SELECT ON postgres_exporter.pg_stat_replication TO postgres_exporter;
EOSQL
