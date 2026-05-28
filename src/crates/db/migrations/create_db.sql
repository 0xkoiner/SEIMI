create table protocols (
    id BIGSERIAL NOT NULL PRIMARY KEY,
    name TEXT VARCHAR(50) NOT NULL UNIQUE,
);

create table protocol_chains (
    protocol_id BIGINT      NOT NULL REFERENCES protocols(id) ON DELETE CASCADE,
    chain       VARCHAR(32) NOT NULL,
    PRIMARY KEY (protocol_id, chain)
);