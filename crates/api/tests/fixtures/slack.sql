--
-- PostgreSQL database dump
--

-- Dumped from database version 17.2 (Debian 17.2-1.pgdg120+1)
-- Dumped by pg_dump version 17.2 (Debian 17.2-1.pgdg120+1)


--
-- Data for Name: channels; Type: TABLE DATA; Schema: slack; Owner: postgres
--

INSERT INTO slack.channels VALUES ('C03ALCPRQSK', 'random', DEFAULT, true, false, false, 1649695925, 'U03B3B30T1A', NULL, false, false, 0, 'random', false, false, false, '{}', false, true, false, false, 1724745422118, '{"value": "", "creator": "", "last_set": 0}', '{"value": "This channel is for... well, everything else. It’s a place for team jokes, spur-of-the-moment ideas, and funny GIFs. Go wild!", "creator": "U03B3B30T1A", "last_set": 1649695925}', '{}', 8);
INSERT INTO slack.channels VALUES ('C080WPASXQW', 'project_sundry', DEFAULT, true, false, false, 1731692341, 'U05RR37L1EH', NULL, false, false, 0, 'project_sundry', false, false, false, '{}', false, true, false, false, 1733166265709, '{"value": "", "creator": "", "last_set": 0}', '{"value": "", "creator": "", "last_set": 0}', '{project_pivot}', 8);

--
-- Data for Name: app_installation_channels; Type: TABLE DATA; Schema: slack; Owner: postgres
--

INSERT INTO slack.app_installation_channels VALUES (1, 'C03ALCPRQSK');
INSERT INTO slack.app_installation_channels VALUES (1, 'C080WPASXQW');

--
-- Data for Name: app_installations; Type: TABLE DATA; Schema: slack; Owner: postgres
--

INSERT INTO slack.app_installations OVERRIDING SYSTEM VALUE VALUES (1, 'xoxb-3368380417254-8143544170368-qENf5WAATUtFKBktAr34Wgav', 1, '2024-12-30 21:00:10.639343', NULL);


--
-- Data for Name: channel_members; Type: TABLE DATA; Schema: slack; Owner: postgres
--

-- Silas
INSERT INTO slack.channel_members VALUES ('C03AUB6DHFY', 'U059Y3EUM7G');
INSERT INTO slack.channel_members VALUES ('C080WPASXQW', 'U059Y3EUM7G');
INSERT INTO slack.channel_members VALUES ('C03ALCPRQSK', 'U059Y3EUM7G');

-- Ryan
INSERT INTO slack.channel_members VALUES ('C080WPASXQW', 'U05MLVD1BEF');


--
-- Data for Name: messages; Type: TABLE DATA; Schema: slack; Owner: postgres
--

INSERT INTO slack.messages OVERRIDING SYSTEM VALUE VALUES (1, 'C080WPASXQW', 'message', 'U059Y3EUM7G', 'Awesome!!', DEFAULT, NULL, NULL, '1696525240.154879');

INSERT INTO slack.messages OVERRIDING SYSTEM VALUE VALUES (2, 'C080WPASXQW', 'message', 'U05MLVD1BEF', 'Do we have social links I can add to the footer for sundry?
Github:
X:
Bluesky:', DEFAULT, '1734722252.820149', NULL, '1734722252.820149');
INSERT INTO slack.messages OVERRIDING SYSTEM VALUE VALUES (3, 'C080WPASXQW', 'message', 'U05RR37L1EH', 'Not for bluesky or x yet. We’re waiting for launch to make those public. We do have a private GitHub for now ', DEFAULT, '1734722252.820149', 'U05MLVD1BEF', '1734722337.323629');

INSERT INTO slack.messages OVERRIDING SYSTEM VALUE VALUES (4, 'C03ALCPRQSK', 'message', 'U05HD15MJF4', 'This could be us', DEFAULT, '1730333466.512629', 'U052HDWP5GV', '1730333494.893229');
INSERT INTO slack.messages OVERRIDING SYSTEM VALUE VALUES (5, 'C03ALCPRQSK', 'message', 'U052HDWP5GV', 'It will be us ', DEFAULT, '1730333466.512629', 'U052HDWP5GV', '1730333596.573649');

--
-- Data for Name: users; Type: TABLE DATA; Schema: slack; Owner: postgres
--

INSERT INTO slack.users VALUES ('U059Y3EUM7G', 'T03AUB6C97G', 'silas', 'Silas');
INSERT INTO slack.users VALUES ('U05MLVD1BEF', 'T03AUB6C97G', 'ryan', 'Ryan');
INSERT INTO slack.users VALUES ('U05RR37L1EH', 'T03AUB6C97G', 'cassandra', 'Cassandra');
INSERT INTO slack.users VALUES ('U05HD15MJF4', 'T03AUB6C97G', 'kevin', 'Kevin');
INSERT INTO slack.users VALUES ('U052HDWP5GV', 'T03AUB6C97G', 'angela', 'Angela');


--
-- Name: app_installations_id_seq; Type: SEQUENCE SET; Schema: slack; Owner: postgres
--

SELECT pg_catalog.setval('slack.app_installations_id_seq', 1, true);


--
-- Name: messages_id_seq; Type: SEQUENCE SET; Schema: slack; Owner: postgres
--

SELECT pg_catalog.setval('slack.messages_id_seq', 317088, true);


--
-- PostgreSQL database dump complete
--
