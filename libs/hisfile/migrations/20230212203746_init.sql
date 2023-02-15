-- Add migration script here
CREATE TABLE file 
  (
    tramp_id INTEGER not null,
    fullpath TEXT not null,
    dirpath TEXT not null,
    filename TEXT not null,  
    last_ref INTEGER not null,
    freq INTEGER not null,
    priority INTEGER not null,
    -- -1:deleted
    --  0:normal
    --  1:saved
    FOREIGN KEY(tramp_id) REFERENCES tramp(id),
    PRIMARY KEY (tramp_id, fullpath)
  );

CREATE TABLE tramp
  (
    id INTEGER not null, 
    alias TEXT ,
    tramp_type TEXT not null,
    tramp_path TEXT not null,
    PRIMARY KEY (id)
  );

INSERT INTO tramp VALUES
 (0, 'local', '', ''),
 (1, 'root', 'sudo', 'root@localhost')
 ;
