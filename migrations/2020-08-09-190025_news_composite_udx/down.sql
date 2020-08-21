-- This file should undo anything in `up.sql`
drop index news_guid;

create unique index news_guid
    on news (guid);