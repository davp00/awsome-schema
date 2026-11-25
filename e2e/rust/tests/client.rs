//! Same cases as `e2e/typescript/surreal/tests` against the generated Rust client.
#![allow(
    clippy::too_many_lines,
    clippy::similar_names,
    clippy::wildcard_imports,
    clippy::items_after_statements,
    clippy::float_cmp
)]

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use awesome_schema_surreal::{
    ClientError, JsonMap, LiveAction, LiveHandle, ManyResult, Session, SingleResult,
};
use e2e_rust::*;
use renderers::SurrealDbRenderer;
use schema_core::SchemaRenderer;
use serde_json::{Value, json};
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::{Client as RemoteClient, Ws};
use surrealdb::opt::auth::Root;
use testcontainers_modules::surrealdb::{SURREALDB_PORT, SurrealDb};
use testcontainers_modules::testcontainers::ContainerAsync;
use testcontainers_modules::testcontainers::ImageExt;
use testcontainers_modules::testcontainers::runners::AsyncRunner;
use tokio::time::timeout;

type Schema = e2e_rust::Client<RemoteClient>;

struct Env {
    _container: ContainerAsync<SurrealDb>,
    client: Schema,
}

#[tokio::test(flavor = "multi_thread")]
async fn rust_client_matches_typescript_client() {
    let Some(env) = boot().await else {
        return;
    };
    let client = &env.client;
    crud(client).await;
    select_where_and_order(client).await;
    relation_count_order(client).await;
    nested_relation_order_and_page(client).await;
    bulk_return_and_upsert(client).await;
    single_return(client).await;
    count_and_group(client).await;
    nested_writes(client).await;
    transactions(client).await;
    raw_query(client).await;
    live(client).await;
}

async fn boot() -> Option<Env> {
    let container = match SurrealDb::default()
        .with_user("root")
        .with_password("root")
        .with_tag(surreal_tag())
        .start()
        .await
    {
        Ok(container) => container,
        Err(error) if docker_unavailable(&error) && !must_run() => {
            eprintln!("skipping rust client e2e: {error}");
            return None;
        }
        Err(error) => panic!("start surrealdb: {error}"),
    };
    let port = container.get_host_port_ipv4(SURREALDB_PORT).await.expect("mapped port");
    let db = Surreal::new::<Ws>(format!("127.0.0.1:{port}")).await.expect("connect");
    db.signin(Root { username: "root".to_owned(), password: "root".to_owned() })
        .await
        .expect("signin");
    db.use_ns("test").use_db("main").await.expect("use ns/db");
    push_schema(&db).await;
    let client = create_client(Arc::new(Session::new(db)));
    Some(Env { _container: container, client })
}

async fn push_schema(db: &Surreal<RemoteClient>) {
    let schema = parser::parse(include_str!("../fixtures/client.schema")).expect("parse fixture");
    let sql = SurrealDbRenderer::new().render_schema(&schema).expect("render schema");
    // Same statement split as the CLI executor. Current Surreal rejects
    // `TYPE record<table>` on `id` (the id field is implicit); other defines still apply.
    for statement in sql.lines().map(str::trim).filter(|line| !line.is_empty()) {
        let mut response =
            db.query(statement).await.unwrap_or_else(|error| panic!("push `{statement}`: {error}"));
        let fatal: Vec<String> = response
            .take_errors()
            .into_values()
            .filter(|error| !error.to_string().contains("not a valid record id key"))
            .map(|error| error.to_string())
            .collect();
        assert!(fatal.is_empty(), "schema push failed on `{statement}`: {fatal:?}");
    }
}

async fn crud(client: &Schema) {
    let email = unique_email("crud");
    let created: User = client.user().create(&user(&email, 30, "crud-e2e")).await.expect("create");
    assert_eq!(created.email, email);
    assert_eq!(created.age, Some(30));
    assert_eq!(created.metadata.source, "crud-e2e");
    assert_eq!(created.tags, vec!["e2e".to_owned()]);

    let found: Option<User> = client.user().find_unique(&by_id(&created.id)).await.expect("find");
    let found = found.expect("created user");
    assert_eq!(found.id, created.id);

    let updated: User = client
        .user()
        .update(&created.id, &UserUpdate { age: Some(31), ..UserUpdate::default() }, None)
        .await
        .expect("update");
    assert_eq!(updated.age, Some(31));

    let _: () = client.user().delete(&created.id, None).await.expect("delete");
    let gone: Option<User> = client.user().find_unique(&by_id(&created.id)).await.expect("gone");
    assert!(gone.is_none());
}

async fn select_where_and_order(client: &Schema) {
    let mark = unique("where");
    let mut ids = Vec::new();
    for age in [10, 20, 30] {
        let email = format!("{mark}-{age}@example.com");
        let row: User =
            client.user().create(&user(&email, age, "where-e2e")).await.expect("create");
        if age == 20 {
            let _: Post = client
                .post()
                .create(&PostCreate {
                    id: None,
                    title: format!("where-post-{mark}"),
                    author: LinkWrite::Id(row.id.clone()),
                    editor: None,
                })
                .await
                .expect("post");
        }
        ids.push(row.id);
    }

    let older: Vec<User> = client
        .user()
        .find_many(&UserFindMany {
            r#where: Some(UserWhere {
                and: Some(vec![
                    email_contains(&mark),
                    UserWhere {
                        age: Some(NumberFilter::Ops(NumberFilterOps {
                            gte: Some(20.0),
                            ..NumberFilterOps::default()
                        })),
                        ..UserWhere::default()
                    },
                ]),
                ..UserWhere::default()
            }),
            order_by: Some(OneOrMany::One(UserOrderBy {
                age: Some(SortDir::Asc),
                ..UserOrderBy::default()
            })),
            ..UserFindMany::default()
        })
        .await
        .expect("where");
    assert_eq!(older.iter().map(|row| row.age).collect::<Vec<_>>(), vec![Some(20), Some(30)]);

    let page: Vec<User> = client
        .user()
        .find_many(&UserFindMany {
            r#where: Some(email_contains(&mark)),
            order_by: Some(OneOrMany::Many(vec![
                UserOrderBy { age: Some(SortDir::Asc), ..UserOrderBy::default() },
                UserOrderBy { email: Some(SortDir::Asc), ..UserOrderBy::default() },
            ])),
            take: Some(2),
            skip: Some(1),
            ..UserFindMany::default()
        })
        .await
        .expect("page");
    assert_eq!(page.iter().map(|row| row.age).collect::<Vec<_>>(), vec![Some(20), Some(30)]);

    let mut vars = JsonMap::new();
    vars.insert("email".to_owned(), json!(format!("{mark}-10@example.com")));
    let sql_rows: Vec<User> = client
        .user()
        .find_many(&UserFindMany {
            where_sql: Some("SELECT * FROM user WHERE email = $email".to_owned()),
            vars: Some(vars),
            ..UserFindMany::default()
        })
        .await
        .expect("where sql");
    assert_eq!(sql_rows.len(), 1);

    let related: Vec<User> = client
        .user()
        .find_many(&UserFindMany {
            r#where: Some(UserWhere {
                posts: Some(RelationFilter::Some {
                    some: Box::new(PostWhere {
                        title: Some(StringFilter::Value(format!("where-post-{mark}"))),
                        ..PostWhere::default()
                    }),
                }),
                ..UserWhere::default()
            }),
            ..UserFindMany::default()
        })
        .await
        .expect("relation some");
    assert_eq!(related.len(), 1);
    assert_eq!(related[0].age, Some(20));

    let projected: Vec<Value> = client
        .user()
        .find_many(&UserFindMany {
            select: Some(UserSelect {
                email: Some(true),
                posts: Some(posts_bag(SortDir::Asc, None, None)),
                ..UserSelect::default()
            }),
            r#where: Some(email_contains(&mark)),
            order_by: Some(OneOrMany::One(UserOrderBy {
                email: Some(SortDir::Asc),
                ..UserOrderBy::default()
            })),
            ..UserFindMany::default()
        })
        .await
        .expect("select");
    assert!(projected.iter().all(|row| row.get("age").is_none()));
    assert!(projected.iter().all(|row| row.get("tags").is_none()));

    for id in ids {
        remove_user(client, &id).await;
    }
}

async fn relation_count_order(client: &Schema) {
    let mark = unique("count-order");
    let mut ids = Vec::new();
    for (label, titles) in [("few", vec!["a"]), ("many", vec!["a", "b", "c"])] {
        let row: User = client
            .user()
            .create(&with_posts(&format!("{mark}-{label}@example.com"), 25, "order-e2e", &titles))
            .await
            .expect("create");
        ids.push((label, row.id));
    }
    let ordered: Vec<User> = client
        .user()
        .find_many(&UserFindMany {
            r#where: Some(email_contains(&mark)),
            order_by: Some(OneOrMany::One(UserOrderBy {
                posts: Some(RelationCountOrder { count: SortDir::Desc }),
                ..UserOrderBy::default()
            })),
            ..UserFindMany::default()
        })
        .await
        .expect("order count");
    assert_eq!(ordered.len(), 2);
    assert!(ordered[0].email.contains("many"));
    assert!(ordered[1].email.contains("few"));
    for (_, id) in ids {
        remove_user(client, &id).await;
    }
}

async fn nested_relation_order_and_page(client: &Schema) {
    let mark = unique("nested");
    let titles = [format!("nord-c-{mark}"), format!("nord-a-{mark}"), format!("nord-b-{mark}")];
    let title_refs: Vec<&str> = titles.iter().map(String::as_str).collect();
    let user: User = client
        .user()
        .create(&with_posts(&format!("{mark}@example.com"), 27, "nested-order-e2e", &title_refs))
        .await
        .expect("create");

    let row: Option<Value> = client
        .user()
        .find_unique(&UserFindUnique {
            r#where: id_where(&user.id),
            select: Some(UserSelect {
                email: Some(true),
                posts: Some(posts_bag(SortDir::Asc, None, None)),
                ..UserSelect::default()
            }),
            vars: None,
        })
        .await
        .expect("nested order");
    let row = row.expect("row");
    assert_eq!(
        titles_of(row.get("posts").expect("posts")),
        vec![format!("nord-a-{mark}"), format!("nord-b-{mark}"), format!("nord-c-{mark}")]
    );
    let posts = row.get("posts").and_then(Value::as_array).expect("posts");
    assert!(posts.iter().all(|post| post.get("author").is_none()));

    let paged: Option<Value> = client
        .user()
        .find_unique(&UserFindUnique {
            r#where: id_where(&user.id),
            select: Some(UserSelect {
                posts: Some(posts_bag(SortDir::Asc, Some(2), Some(1))),
                ..UserSelect::default()
            }),
            vars: None,
        })
        .await
        .expect("nested page");
    assert_eq!(
        titles_of(paged.expect("page").get("posts").expect("posts")),
        vec![format!("nord-b-{mark}"), format!("nord-c-{mark}")]
    );
    remove_user(client, &user.id).await;
}

async fn bulk_return_and_upsert(client: &Schema) {
    let mark = unique("bulk");
    let created = client
        .user()
        .create_many(&UserCreateMany {
            data: vec![
                user(&format!("{mark}-a@example.com"), 21, "bulk-e2e"),
                user(&format!("{mark}-b@example.com"), 22, "bulk-e2e"),
            ],
            select: None,
            returning: None,
        })
        .await
        .expect("create many");
    let ManyResult::Count { count } = created else {
        panic!("expected count, got {created:?}");
    };
    assert_eq!(count, 2);

    let after = client
        .user()
        .create_many(&UserCreateMany {
            data: vec![user(&format!("{mark}-c@example.com"), 23, "return-e2e")],
            select: None,
            returning: Some(MutationReturnMode::After),
        })
        .await
        .expect("create many after");
    let ManyResult::Rows(rows) = after else {
        panic!("expected rows, got {after:?}");
    };
    assert_eq!(rows.len(), 1);
    assert_eq!(
        rows[0].get("email").and_then(Value::as_str),
        Some(format!("{mark}-c@example.com").as_str())
    );

    let rejected = client
        .user()
        .create_many(&UserCreateMany {
            data: vec![user(&format!("{mark}-bad@example.com"), 1, "return-e2e")],
            select: None,
            returning: Some(MutationReturnMode::Before),
        })
        .await
        .expect_err("before");
    assert!(rejected.to_string().contains("BEFORE is not supported"));

    let updated = client
        .user()
        .update_many(&UserUpdateMany {
            r#where: email_contains(&mark),
            data: UserUpdate { age: Some(30), ..UserUpdate::default() },
            select: None,
            returning: Some(MutationReturnMode::After),
        })
        .await
        .expect("update many");
    let ManyResult::Rows(rows) = updated else {
        panic!("expected rows, got {updated:?}");
    };
    assert!(rows.len() >= 3);
    assert!(rows.iter().all(|row| number(row.get("age").expect("age")) == 30.0));

    let diff = client
        .user()
        .update_many(&UserUpdateMany {
            r#where: email_contains(&mark),
            data: UserUpdate {
                tags: Some(vec!["e2e".to_owned(), "patched".to_owned()]),
                ..UserUpdate::default()
            },
            select: None,
            returning: Some(MutationReturnMode::Diff),
        })
        .await
        .expect("diff");
    let ManyResult::Rows(parts) = diff else {
        panic!("expected diff rows, got {diff:?}");
    };
    assert_ne!(parts.len(), 0);

    let none = client
        .user()
        .update_many(&UserUpdateMany {
            r#where: email_contains(&mark),
            data: UserUpdate { age: Some(31), ..UserUpdate::default() },
            select: None,
            returning: Some(MutationReturnMode::None),
        })
        .await
        .expect("none");
    let ManyResult::Count { count } = none else {
        panic!("expected count, got {none:?}");
    };
    assert!(count >= 3);

    let email = format!("{mark}-upsert@example.com");
    let first = client
        .user()
        .upsert(&UserUpsert {
            r#where: UserWhereUnique::Email { email: email.clone() },
            create: user(&email, 20, "upsert-e2e"),
            update: UserUpdate { age: Some(41), ..UserUpdate::default() },
            select: None,
            returning: None,
        })
        .await
        .expect("upsert create");
    let SingleResult::Row(Some(_)) = first else {
        panic!("expected created row, got {first:?}");
    };
    let second = client
        .user()
        .upsert(&UserUpsert {
            r#where: UserWhereUnique::Email { email: email.clone() },
            create: user(&email, 20, "upsert-e2e"),
            update: UserUpdate { age: Some(41), ..UserUpdate::default() },
            select: None,
            returning: None,
        })
        .await
        .expect("upsert update");
    let SingleResult::Row(Some(value)) = second else {
        panic!("expected updated row, got {second:?}");
    };
    let updated: User = awesome_schema_surreal::decode(value).expect("decode upsert");
    assert_eq!(updated.age, Some(41));
    let matches =
        client.user().count(&UserCount { r#where: Some(email_eq(&email)) }).await.expect("count");
    assert_eq!(matches, 1);

    let removed = client
        .user()
        .delete_many(&UserDeleteMany {
            r#where: email_contains(&mark),
            select: None,
            returning: Some(MutationReturnMode::None),
        })
        .await
        .expect("delete many");
    let ManyResult::Count { count } = removed else {
        panic!("expected count, got {removed:?}");
    };
    assert!(count >= 4);
}

async fn single_return(client: &Schema) {
    let mark = unique("single");
    let email = format!("{mark}@example.com");
    let user: User = client.user().create(&user(&email, 28, "single-e2e")).await.expect("create");
    let post: Post = client
        .post()
        .create(&PostCreate {
            id: None,
            title: format!("single-{mark}"),
            author: LinkWrite::Id(user.id.clone()),
            editor: None,
        })
        .await
        .expect("post");
    let like: Likes = client
        .likes()
        .create(&LikesCreate {
            id: None,
            r#in: LinkWrite::Id(user.id.clone()),
            r#out: LinkWrite::Id(post.id.clone()),
            score: 7,
        })
        .await
        .expect("like");
    assert_eq!(like.score, 7);

    let before: User = client
        .user()
        .update(
            &user.id,
            &UserUpdate { age: Some(29), ..UserUpdate::default() },
            Some(&UserMutation {
                returning: Some(MutationReturnMode::Before),
                ..UserMutation::default()
            }),
        )
        .await
        .expect("before");
    assert_eq!(before.age, Some(28));

    let projected: Value = client
        .user()
        .update(
            &user.id,
            &UserUpdate::default(),
            Some(&UserMutation {
                select: Some(UserSelect {
                    id: Some(true),
                    email: Some(true),
                    age: Some(true),
                    posts: Some(NestedSelect {
                        select: Some(Box::new(PostSelect {
                            id: Some(true),
                            title: Some(true),
                            author: Some(NestedSelect {
                                select: Some(Box::new(UserSelect {
                                    email: Some(true),
                                    ..UserSelect::default()
                                })),
                                ..NestedSelect::default()
                            }),
                            ..PostSelect::default()
                        })),
                        ..NestedSelect::default()
                    }),
                    liked: Some(NestedSelect {
                        select: Some(Box::new(LikesSelect {
                            id: Some(true),
                            score: Some(true),
                            r#out: Some(NestedSelect {
                                select: Some(Box::new(PostSelect {
                                    id: Some(true),
                                    title: Some(true),
                                    ..PostSelect::default()
                                })),
                                ..NestedSelect::default()
                            }),
                            ..LikesSelect::default()
                        })),
                        ..NestedSelect::default()
                    }),
                    ..UserSelect::default()
                }),
                returning: None,
            }),
        )
        .await
        .expect("select");
    assert!(projected.get("tags").is_none());
    assert_eq!(projected.get("email").and_then(Value::as_str), Some(email.as_str()));
    assert_eq!(number(projected.get("age").expect("age")), 29.0);
    let posts = titles_of(projected.get("posts").expect("posts"));
    assert_eq!(posts, vec![format!("single-{mark}")]);
    let likes = projected.get("liked").and_then(Value::as_array).expect("liked");
    assert_eq!(number(likes[0].get("score").expect("score")), 7.0);
    assert_eq!(
        likes[0].get("out").and_then(|row| row.get("title")).and_then(Value::as_str),
        Some(format!("single-{mark}").as_str())
    );

    let deleted: User = client
        .user()
        .delete(
            &user.id,
            Some(&UserMutation {
                returning: Some(MutationReturnMode::Before),
                ..UserMutation::default()
            }),
        )
        .await
        .expect("delete before");
    assert_eq!(deleted.email, email);
    let gone: Option<User> = client.user().find_unique(&by_id(&user.id)).await.expect("gone");
    assert!(gone.is_none());
}

async fn count_and_group(client: &Schema) {
    let mark = unique("agg");
    let mut ids = Vec::new();
    for age in [10, 10, 20, 30, 30, 30] {
        let row: User = client
            .user()
            .create(&user(&format!("{mark}-{age}-{}@example.com", ids.len()), age, "aggregate-e2e"))
            .await
            .expect("create");
        ids.push(row.id);
    }
    let filtered = client
        .user()
        .count(&UserCount { r#where: Some(email_contains(&mark)) })
        .await
        .expect("count");
    assert_eq!(filtered, 6);
    let older = client
        .user()
        .count(&UserCount {
            r#where: Some(UserWhere {
                and: Some(vec![
                    email_contains(&mark),
                    UserWhere {
                        age: Some(NumberFilter::Ops(NumberFilterOps {
                            gte: Some(20.0),
                            ..NumberFilterOps::default()
                        })),
                        ..UserWhere::default()
                    },
                ]),
                ..UserWhere::default()
            }),
        })
        .await
        .expect("older");
    assert_eq!(older, 4);
    let total = client.user().count(&UserCount::default()).await.expect("total");
    assert!(total >= 6);

    let groups: Vec<Value> = client
        .user()
        .group_by(&UserGroupBy {
            by: vec![UserScalarField::Age],
            r#where: Some(email_contains(&mark)),
            count: Some(CountArg::Flag(true)),
            sum: Some(json!({"age": true})),
            avg: Some(json!({"age": true})),
            min: Some(json!({"age": true})),
            max: Some(json!({"age": true})),
            having: None,
            order_by: Some(OneOrMany::One(UserGroupOrder {
                count: Some(json!({"_all": "desc"})),
                ..UserGroupOrder::default()
            })),
            take: Some(2),
            skip: None,
        })
        .await
        .expect("group");
    assert_eq!(groups.len(), 2);
    assert_eq!(number(groups[0].get("age").expect("age")), 30.0);
    assert_eq!(bucket(&groups[0], "_count", "_all"), 3.0);
    assert_eq!(bucket(&groups[0], "_sum", "age"), 90.0);
    assert_eq!(bucket(&groups[0], "_avg", "age"), 30.0);
    assert_eq!(bucket(&groups[0], "_min", "age"), 30.0);
    assert_eq!(bucket(&groups[0], "_max", "age"), 30.0);
    assert_eq!(number(groups[1].get("age").expect("age")), 10.0);
    assert_eq!(bucket(&groups[1], "_count", "_all"), 2.0);

    let having: Vec<Value> = client
        .user()
        .group_by(&UserGroupBy {
            by: vec![UserScalarField::Age],
            r#where: Some(email_contains(&mark)),
            count: Some(CountArg::Flag(true)),
            sum: None,
            avg: None,
            min: None,
            max: None,
            having: Some(UserHaving {
                agg_count: Some(json!({"_all": {"gt": 1}})),
                ..UserHaving::default()
            }),
            order_by: Some(OneOrMany::One(UserGroupOrder {
                age: Some(SortDir::Asc),
                ..UserGroupOrder::default()
            })),
            take: None,
            skip: None,
        })
        .await
        .expect("having count");
    let ages: Vec<f64> = having.iter().map(|row| number(row.get("age").expect("age"))).collect();
    assert_eq!(ages, vec![10.0, 30.0]);

    let empty = client
        .user()
        .group_by::<Value>(&UserGroupBy {
            by: Vec::new(),
            r#where: None,
            count: Some(CountArg::Flag(true)),
            sum: None,
            avg: None,
            min: None,
            max: None,
            having: None,
            order_by: None,
            take: None,
            skip: None,
        })
        .await
        .expect_err("empty by");
    assert!(empty.to_string().contains("by must not be empty"));
    let missing = client
        .user()
        .group_by::<Value>(&UserGroupBy {
            by: vec![UserScalarField::Age],
            r#where: None,
            count: None,
            sum: None,
            avg: None,
            min: None,
            max: None,
            having: None,
            order_by: None,
            take: None,
            skip: None,
        })
        .await
        .expect_err("missing aggregate");
    assert!(missing.to_string().contains("at least one aggregate"));

    for id in ids {
        remove_user(client, &id).await;
    }
}

async fn nested_writes(client: &Schema) {
    let mark = unique("nested-write");
    let created: User = client
        .user()
        .create(&with_posts(
            &format!("{mark}-posts@example.com"),
            31,
            "nested-write-e2e",
            &["nw-a", "nw-b"],
        ))
        .await
        .expect("nested posts");
    let posts: Vec<Post> = client
        .post()
        .find_many(&PostFindMany {
            r#where: Some(PostWhere {
                author: Some(Box::new(id_where(&created.id))),
                ..PostWhere::default()
            }),
            order_by: Some(OneOrMany::One(PostOrderBy {
                title: Some(SortDir::Asc),
                ..PostOrderBy::default()
            })),
            ..PostFindMany::default()
        })
        .await
        .expect("posts");
    assert_eq!(
        posts.iter().map(|post| post.title.as_str()).collect::<Vec<_>>(),
        vec!["nw-a", "nw-b"]
    );
    remove_user(client, &created.id).await;

    let owner: User = client
        .user()
        .create(&user(&format!("{mark}-owner@example.com"), 22, "nested-write-e2e"))
        .await
        .expect("owner");
    let other: User = client
        .user()
        .create(&user(&format!("{mark}-other@example.com"), 23, "nested-write-e2e"))
        .await
        .expect("other");
    let post: Post = client
        .post()
        .create(&PostCreate {
            id: None,
            title: format!("nw-conn-{mark}"),
            author: LinkWrite::Id(other.id.clone()),
            editor: None,
        })
        .await
        .expect("post");
    let _: User = client
        .user()
        .update(
            &owner.id,
            &UserUpdate {
                posts: Some(json!({"connect": [{"id": post.id}]})),
                ..UserUpdate::default()
            },
            None,
        )
        .await
        .expect("connect");
    let moved: Option<Post> = client
        .post()
        .find_unique(&PostFindUnique {
            r#where: PostWhere {
                id: Some(IdFilter::Value(post.id.clone())),
                ..PostWhere::default()
            },
            select: None,
            vars: None,
        })
        .await
        .expect("moved");
    assert_eq!(moved.expect("post").author, owner.id);
    let rejected = client
        .user()
        .update::<User>(
            &owner.id,
            &UserUpdate {
                posts: Some(json!({"disconnect": [{"id": post.id}]})),
                ..UserUpdate::default()
            },
            None,
        )
        .await
        .expect_err("disconnect");
    assert!(rejected.to_string().contains("cannot disconnect required back-link"));
    remove_user(client, &owner.id).await;
    remove_user(client, &other.id).await;

    let mut liked = user(&format!("{mark}-liked@example.com"), 40, "nested-write-e2e");
    liked.liked = Some(json!({
        "create": [{ "score": 8, "out": { "create": { "title": format!("nw-liked-{mark}") } } }]
    }));
    let liker: User = client.user().create(&liked).await.expect("liked");
    let made: Vec<Post> = client
        .post()
        .find_many(&PostFindMany {
            r#where: Some(PostWhere {
                title: Some(StringFilter::Value(format!("nw-liked-{mark}"))),
                ..PostWhere::default()
            }),
            ..PostFindMany::default()
        })
        .await
        .expect("liked post");
    assert_eq!(made.len(), 1);
    remove_user(client, &liker.id).await;
}

async fn transactions(client: &Schema) {
    let mark = unique("tx");
    let email = format!("{mark}@example.com");
    let row = user(&email, 40, "tx-e2e");
    let title = format!("tx-{mark}");
    let (user_id, post_id) = client
        .transaction(|tx| {
            let row = row.clone();
            let title = title.clone();
            async move {
                let created: User = tx.user().create(&row).await?;
                let post: Post = tx
                    .post()
                    .create(&PostCreate {
                        id: None,
                        title,
                        author: LinkWrite::Id(created.id.clone()),
                        editor: None,
                    })
                    .await?;
                Ok((created.id, post.id))
            }
        })
        .await
        .expect("commit");
    let saved: Option<User> = client.user().find_unique(&by_id(&user_id)).await.expect("user");
    let saved_post: Option<Post> = client
        .post()
        .find_unique(&PostFindUnique {
            r#where: PostWhere {
                id: Some(IdFilter::Value(post_id.clone())),
                ..PostWhere::default()
            },
            select: None,
            vars: None,
        })
        .await
        .expect("post");
    assert_eq!(saved.expect("user").email, email);
    assert_eq!(saved_post.expect("post").title, title);
    remove_user(client, &user_id).await;

    let rollback_email = format!("{mark}-rollback@example.com");
    let rollback_row = user(&rollback_email, 1, "tx-e2e");
    let failed = client
        .transaction(|tx| {
            let rollback_row = rollback_row.clone();
            async move {
                let _: User = tx.user().create(&rollback_row).await?;
                Err::<(), ClientError>(ClientError::new("rollback".to_owned()))
            }
        })
        .await;
    assert!(failed.is_err());
    let leftover: Vec<User> = client
        .user()
        .find_many(&UserFindMany {
            r#where: Some(email_eq(&rollback_email)),
            ..UserFindMany::default()
        })
        .await
        .expect("leftover");
    assert!(leftover.is_empty());
}

async fn raw_query(client: &Schema) {
    let email = unique_email("raw");
    let created: User = client.user().create(&user(&email, 18, "raw-e2e")).await.expect("create");
    let mut vars = JsonMap::new();
    vars.insert("email".to_owned(), json!(email));
    let rows: Vec<User> = client
        .query_raw("SELECT * FROM user WHERE email = $email", Some(&vars))
        .await
        .expect("query raw");
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].id, created.id);
    let executed = client.execute_raw("RETURN 7", None).await.expect("execute raw");
    assert!(executed.to_string().contains('7'));
    remove_user(client, &created.id).await;
}

async fn live(client: &Schema) {
    let email = unique_email("live");
    let mut handle = client.user().live().await.expect("live");
    let created: User = client.user().create(&user(&email, 21, "live-e2e")).await.expect("create");
    wait_live(&mut handle, &email, LiveAction::Create).await;
    let _: User = client
        .user()
        .update(&created.id, &UserUpdate { age: Some(22), ..UserUpdate::default() }, None)
        .await
        .expect("update");
    wait_live(&mut handle, &email, LiveAction::Update).await;
    let _: () = client.user().delete(&created.id, None).await.expect("delete");
    wait_live(&mut handle, &email, LiveAction::Delete).await;
    handle.kill().await.expect("kill");
}

async fn wait_live(handle: &mut LiveHandle, email: &str, action: LiveAction) {
    let deadline = tokio::time::Instant::now() + Duration::from_secs(20);
    loop {
        let remaining = deadline.saturating_duration_since(tokio::time::Instant::now());
        assert!(!remaining.is_zero(), "live event timed out for {email}");
        let event =
            timeout(remaining, handle.recv()).await.expect("live wait").expect("live closed");
        let Some(found) = event.value.get("email").and_then(Value::as_str) else {
            continue;
        };
        if found == email {
            assert_eq!(event.action, action);
            return;
        }
    }
}

async fn remove_user(client: &Schema, id: &str) {
    let likes: Vec<Likes> = client
        .likes()
        .find_many(&LikesFindMany {
            r#where: Some(LikesWhere {
                r#in: Some(Box::new(id_where(id))),
                ..LikesWhere::default()
            }),
            ..LikesFindMany::default()
        })
        .await
        .expect("likes");
    for like in likes {
        let _: () = client.likes().delete(&like.id, None).await.expect("delete like");
    }
    let posts: Vec<Post> = client
        .post()
        .find_many(&PostFindMany {
            r#where: Some(PostWhere {
                author: Some(Box::new(id_where(id))),
                ..PostWhere::default()
            }),
            ..PostFindMany::default()
        })
        .await
        .expect("posts");
    for post in posts {
        let _: () = client.post().delete(&post.id, None).await.expect("delete post");
    }
    let existing: Option<User> = client.user().find_unique(&by_id(id)).await.expect("user");
    if existing.is_some() {
        let _: () = client.user().delete(id, None).await.expect("delete user");
    }
}

fn user(email: &str, age: i64, source: &str) -> UserCreate {
    UserCreate {
        id: None,
        email: email.to_owned(),
        age: Some(age),
        metadata: UserMetadata { source: source.to_owned(), ..UserMetadata::default() },
        tags: vec!["e2e".to_owned()],
        posts: None,
        edited: None,
        liked: None,
    }
}

fn with_posts(email: &str, age: i64, source: &str, titles: &[&str]) -> UserCreate {
    let mut row = user(email, age, source);
    row.posts = Some(json!({
        "create": titles.iter().map(|title| json!({"title": title})).collect::<Vec<_>>()
    }));
    row
}

fn email_contains(fragment: &str) -> UserWhere {
    UserWhere {
        email: Some(StringFilter::Ops(StringFilterOps {
            contains: Some(fragment.to_owned()),
            ..StringFilterOps::default()
        })),
        ..UserWhere::default()
    }
}

fn email_eq(email: &str) -> UserWhere {
    UserWhere { email: Some(StringFilter::Value(email.to_owned())), ..UserWhere::default() }
}

fn id_where(id: &str) -> UserWhere {
    UserWhere { id: Some(IdFilter::Value(id.to_owned())), ..UserWhere::default() }
}

fn by_id(id: &str) -> UserFindUnique {
    UserFindUnique { r#where: id_where(id), select: None, vars: None }
}

fn posts_bag(
    order: SortDir,
    take: Option<u64>,
    skip: Option<u64>,
) -> NestedSelect<PostSelect, PostOrderBy> {
    NestedSelect {
        select: Some(Box::new(PostSelect {
            id: Some(true),
            title: Some(true),
            ..PostSelect::default()
        })),
        order_by: Some(PostOrderBy { title: Some(order), ..PostOrderBy::default() }),
        take,
        skip,
    }
}

fn titles_of(value: &Value) -> Vec<String> {
    value
        .as_array()
        .unwrap_or_else(|| panic!("expected array, got {value}"))
        .iter()
        .map(|row| {
            row.get("title")
                .and_then(Value::as_str)
                .unwrap_or_else(|| panic!("title missing in {row}"))
                .to_owned()
        })
        .collect()
}

fn number(value: &Value) -> f64 {
    if let Some(number) = value.as_f64() {
        return number;
    }
    if let Some(number) = value.as_i64() {
        return f64::from(i32::try_from(number).expect("number fits i32"));
    }
    if let Some(number) = value.as_u64() {
        return f64::from(u32::try_from(number).expect("number fits u32"));
    }
    panic!("expected number, got {value}");
}

fn bucket(row: &Value, name: &str, field: &str) -> f64 {
    let object = row
        .get(name)
        .and_then(Value::as_object)
        .unwrap_or_else(|| panic!("{name} missing in {row}"));
    number(object.get(field).unwrap_or_else(|| panic!("{field} missing in {row}")))
}

fn unique(label: &str) -> String {
    static NEXT: AtomicU64 = AtomicU64::new(0);
    let n = NEXT.fetch_add(1, Ordering::Relaxed);
    format!("{label}-{n}")
}

fn unique_email(label: &str) -> String {
    format!("{}@example.com", unique(label))
}

fn surreal_tag() -> String {
    if let Ok(image) = std::env::var("SURREAL_IMAGE") {
        let image = image.trim();
        if !image.is_empty() {
            return match image.rsplit_once(':') {
                Some((_, tag)) => tag.to_owned(),
                None => image.to_owned(),
            };
        }
    }
    if let Ok(tag) = std::env::var("SURREAL_TAG") {
        let tag = tag.trim();
        if !tag.is_empty() {
            return tag.to_owned();
        }
    }
    "v4".to_owned()
}

fn must_run() -> bool {
    std::env::var("CI").ok().as_deref() == Some("true")
        || std::env::var("REQUIRE_SURREAL").ok().as_deref() == Some("1")
}

fn docker_unavailable(error: &impl std::fmt::Display) -> bool {
    let message = error.to_string().to_lowercase();
    message.contains("socketnotfounderror")
        || message.contains("docker.sock")
        || message.contains("client error (connect)")
        || message.contains("connection refused")
        || message.contains("cannot connect")
        || message.contains("docker daemon")
}
