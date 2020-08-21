-- Your SQL goes here
drop index news_guid;

create unique index news_guid
	on news (guid, source_id);