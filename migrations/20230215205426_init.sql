CREATE TABLE tramp
  (
    id SMALLSERIAL not null, 
    tramp_prefix TEXT not null,
    PRIMARY KEY (id)
  );
CREATE UNIQUE INDEX tramp_prefix_index ON tramp(tramp_prefix);

CREATE TABLE file 
  (
    tramp_id SMALLINT not null,
    fullpath TEXT not null,
    dirpath TEXT not null,
    filename TEXT not null,  
    last_ref BIGINT not null,
    freq INTEGER not null,
    deleted BOOL not null,
    status INTEGER not null,
    --  1:filtered
    --  2:normal
    --  3:favourite
    FOREIGN KEY(tramp_id) REFERENCES tramp(id),
    PRIMARY KEY (tramp_id, fullpath)
  );
CREATE INDEX file_indicies ON file(deleted, tramp_id, status);

-- We force local files have tramp id 0 for performance optimization 
INSERT INTO tramp VALUES (0, '');



