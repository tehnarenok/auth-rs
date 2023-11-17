CREATE TABLE "auth_user" (
	"id"				text not null,
	"email"				text unique,
	"email_verified"	boolean not null,
	"login"				text unique,
	"external_id"		text,
	"first_name"		text not null,
	"last_name"			text not null,
	"password"			text,

	"created_at"		timestamptz not null,
	"updated_at"		timestamptz not null,

	PRIMARY KEY ("id"),
	CONSTRAINT valid_auth_user CHECK ("login" ~* '^[a-zA-Z0-9_-]{8,}$')
);

CREATE TABLE "auth_link" (
	"id"				text not null,
	"external_id"		text not null,
	"provider"			text not null,
	"user_id"			text not null,
	"metadata"			jsonb,

	"created_at"		timestamptz not null,
	"updated_at"		timestamptz not null,


	PRIMARY KEY ("id"),
	FOREIGN KEY ("user_id")	REFERENCES "auth_user" ("id") ON DELETE CASCADE
);

CREATE TABLE "auth_session" (
	"id"				text not null,
	"user_id"			text not null,
	"refresh_id"		text not null,
	"ip"				cidr not null,
	"user_agent"		text not null,

	"created_at"		timestamptz not null,
	"updated_at"		timestamptz not null,
	"expired_at"		timestamptz not null,

	PRIMARY KEY ("id"),
	FOREIGN KEY ("user_id") REFERENCES "auth_user" ("id") ON DELETE CASCADE	
);
