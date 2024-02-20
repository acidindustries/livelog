-- Add migration script here
DROP TABLE IF EXISTS logs;
CREATE TABLE logs (
    id CHAR(36) PRIMARY KEY, /* oh boy, here we go */
    date DateTime DEFAULT CURRENT_TIMESTAMP,
    type VARCHAR(50) NOT NULL,
    data TEXT
);

CREATE TRIGGER GenerateGuid
AFTER INSERT ON logs
FOR EACH ROW
WHEN (NEW.id IS NULL)
BEGIN
    UPDATE logs SET id = (select hex( randomblob(4)) || '-' || hex( randomblob(2))
             || '-' || '4' || substr( hex( randomblob(2)), 2) || '-'
             || substr('AB89', 1 + (abs(random()) % 4) , 1)  ||
             substr(hex(randomblob(2)), 2) || '-' || hex(randomblob(6)) )
    WHERE rowid = NEW.rowid;
END;

INSERT INTO logs (type, data) VALUES ('Message', '{"message":"Test"}');
