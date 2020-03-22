BEGIN;

DELETE FROM
   users;

DELETE FROM
   files;

DELETE FROM
   access_user;

DELETE FROM
   roles;

DELETE FROM
   access;

COMMIT;