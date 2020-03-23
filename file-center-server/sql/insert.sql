BEGIN TRANSACTION;

INSERT INTO
    roles (role_name)
VALUES
('ROLE_ADMIN');

INSERT INTO
    roles (role_name)
VALUES
('ROLE_USER');

INSERT INTO
    access (access_type)
VALUES
('READ');

INSERT INTO
    access (access_type)
VALUES
('WRITE');
-- Add Admin Users
INSERT INTO
    users (username, password, email, role_id)
VALUES
(
        'user-foo',
        '$2b$04$Yi6hvlUD7YpFE9seGQEGrO720TPPgAtNPyzK81kK8CNeV6z.uCfcC',
        'foo-mail-sample@outlook.com',
        '2'
    );

INSERT INTO
    users (username, password, email, role_id)
VALUES
(
        'user-bar',
        '$2b$04$Yi6hvlUD7YpFE9seGQEGrO720TPPgAtNPyzK81kK8CNeV6z.uCfcC',
        'bar-mail-sample@gmail.com',
        '2'
    );

COMMIT;