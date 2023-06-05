-- File: V1__initial.sql

create table if not exists public.users
(
    user_id integer generated always as identity
        constraint user_pk
            primary key,
    email   text not null
);

create table if not exists public.poll
(
    poll_id   integer generated always as identity
        constraint poll_pk
            primary key,
    poll_name text
);

create table if not exists public.poll_options
(
    option_id    integer generated always as identity
        constraint poll_options_pk
            primary key,
    option_name  text,
    option_order integer not null
);

create table if not exists public.vote
(
    vote_id    integer generated always as identity,
    created_at timestamp not null,
    poll_id    integer   not null
        constraint vote_poll_poll_id_fk
            references public.poll,
    user_id    integer   not null
        constraint vote_users_user_id_fk
            references public.users,
    option_id  integer   not null
        constraint vote_poll_options_option_id_fk
            references public.poll_options
);

create index if not exists vote_created_at_idx
    on public.vote (created_at desc);


-- Create the hypertable
SELECT create_hypertable('vote', 'created_at');

-- Insert users
-- INSERT INTO users (email) VALUES ('matheus@timescale.com');
-- INSERT INTO users (email) VALUES ('matheus+dev@timescale.com');
-- INSERT INTO users (email) VALUES ('matheus+test@timescale.com');

-- -- Insert some data
-- INSERT INTO poll (poll_name) VALUES ('first poll');
-- INSERT INTO poll_options (option_name, option_order) VALUES ('Option A', '1');
-- INSERT INTO poll_options (option_name, option_order) VALUES ('Option B', '2');
-- INSERT INTO poll_options (option_name, option_order) VALUES ('Option C', '3');
-- INSERT INTO polls (poll_id, option_id) VALUES (1, 1);
-- INSERT INTO polls (poll_id, option_id) VALUES (1, 2);
-- INSERT INTO polls (poll_id, option_id) VALUES (1, 3);


-- insert into vote (created_at, poll_id, user_id, option_id, polls_id) VALUES (now(), 1, 1, 1, 1);
-- insert into vote (created_at, poll_id, user_id, option_id, polls_id) VALUES (now(), 1, 2, 1, 1);
-- insert into vote (created_at, poll_id, user_id, option_id, polls_id) VALUES (now(), 1, 3, 2, 2);

-- insert into vote (created_at, poll_id, user_id, option_id, polls_id) VALUES (now(), 1, 1, 1, 1);
-- insert into vote (created_at, poll_id, user_id, option_id, polls_id) VALUES (now(), 1, 2, 1, 1);
-- insert into vote (created_at, poll_id, user_id, option_id, polls_id) VALUES (now(), 1, 3, 2, 2);

-- insert into vote (created_at, poll_id, user_id, option_id, polls_id) VALUES (now(), 1, 1, 1, 1);
-- insert into vote (created_at, poll_id, user_id, option_id, polls_id) VALUES (now(), 1, 2, 1, 1);
-- insert into vote (created_at, poll_id, user_id, option_id, polls_id) VALUES (now(), 1, 3, 2, 2);

-- insert into vote (created_at, poll_id, user_id, option_id, polls_id) VALUES (now(), 1, 1, 1, 1);
-- insert into vote (created_at, poll_id, user_id, option_id, polls_id) VALUES (now(), 1, 2, 1, 1);
-- insert into vote (created_at, poll_id, user_id, option_id, polls_id) VALUES (now(), 1, 3, 2, 2);

-- insert into users (email) values ('mematheuslc@gmail.com');
-- insert into users (email) values ('mematheuslc+test@gmail.com');
-- insert into users (email) values ('mematheuslc+test3@gmail.com');

-- -- Count votes by 1 day
-- select time_bucket('1 day', created_at) as bucket,
--    count(option_id),
--    option_id
-- from vote
-- GROUP BY bucket, option_id;

-- -- Count votes by 1 minute
-- select time_bucket('1 minute', created_at) as bucket,
--    count(option_id),
--    option_id
-- from vote
-- GROUP BY bucket, option_id;