use codegen::CodeGenerator;
use codegen_typescript::TypeScriptGenerator;
use core::{FieldType, NamingCase};
use parser::parse;

const FIXTURE: &str = r#"
datasource db {
  provider = "surrealdb"
  url = "ws://127.0.0.1:8000"
}

naming {
  tables = "snake_case"
}

type UserMetadata @flexible {
  user_id int?
  source string
}

model User {
  id @id
  email string @unique
  age int?
  metadata UserMetadata
  tags string[]
  posts Post[] @link("PostAuthor")
  liked Post[] @relation("Likes")
  @@table(schemafull)
}

model Post {
  id @id
  title string
  author User @link("PostAuthor") @onDelete(Cascade)
  @@table(schemafull)
}

edge Likes {
  in User
  out Post
  score int
  @@table(schemafull)
}
"#;

fn generate_fixture() -> String {
    let schema = parse(FIXTURE).expect("parse fixture");
    CodeGenerator::generate(&TypeScriptGenerator::new(), &schema).expect("generate")
}

#[test]
fn rejects_unimplemented_typescript_provider() {
    let source = FIXTURE.replacen("provider = \"surrealdb\"", "provider = \"postgres\"", 1);
    let schema = parse(&source).expect("parse");
    let error = CodeGenerator::generate(&TypeScriptGenerator::new(), &schema)
        .expect_err("postgres is not implemented");
    let message = error.to_string();
    assert!(message.contains("postgres"), "{message}");
    assert!(message.contains("surrealdb"), "{message}");
    assert!(!message.contains("RELATE"), "{message}");
}

#[test]
fn default_generator_is_constructible() {
    assert_eq!(CodeGenerator::language(&TypeScriptGenerator::new()), "typescript");
}

#[test]
fn emits_record_id_helper_and_tables() {
    let output = generate_fixture();
    assert!(output.contains("from \"awesome-schema\""));
    assert!(output.contains("type RecordId,"));
    assert!(output.contains("recordId,"));
    assert!(output.contains("parseRecordId,"));
    assert!(output.contains("export const Tables = {"));
    assert!(output.contains("User: \"user\""));
    assert!(output.contains("Post: \"post\""));
    assert!(output.contains("Likes: \"likes\""));
}

#[test]
fn emits_dual_shapes_for_models_and_edges() {
    let output = generate_fixture();
    assert!(output.contains("export type User = {"));
    assert!(output.contains("export type UserSelected = {"));
    assert!(output.contains("export type Post = {"));
    assert!(output.contains("export type PostSelected = {"));
    assert!(output.contains("export type Likes = {"));
    assert!(output.contains("export type LikesSelected = {"));
}

#[test]
fn record_shape_uses_record_id_for_id_and_stored_links() {
    let output = generate_fixture();
    // id branded by table
    assert!(output.contains("id: RecordId<\"user\">"));
    assert!(output.contains("id: RecordId<\"post\">"));
    // stored link on Post
    assert!(output.contains("author: RecordId<\"user\">"));
    // Likes endpoints
    assert!(output.contains("in: RecordId<\"user\">"));
    assert!(output.contains("out: RecordId<\"post\">"));
}

#[test]
fn computed_link_and_relation_omitted_on_record_present_on_selected() {
    let output = generate_fixture();
    // Extract User record block roughly
    let user_start = output.find("export type User = {").expect("User");
    let user_end = output.find("export type UserSelected = {").expect("UserSelected");
    let user_record = &output[user_start..user_end];
    assert!(!user_record.contains("posts"));
    assert!(!user_record.contains("liked"));

    let selected_start = user_end;
    let selected_end = output.find("export type UserCreateScalars = {").expect("UserCreateScalars");
    let user_selected = &output[selected_start..selected_end];
    assert!(user_selected.contains("posts?: PostSelected[]"));
    assert!(user_selected.contains("liked?: LikesSelected[]"));
}

#[test]
fn selected_shapes_populate_links_and_edge_endpoints() {
    let output = generate_fixture();
    assert!(output.contains("author: UserSelected"));
    assert!(output.contains("in: UserSelected"));
    assert!(output.contains("out: PostSelected"));
}

#[test]
fn emits_object_type_and_skips_dotted_nested_fields() {
    let output = generate_fixture();
    assert!(output.contains("export type UserMetadata = {"));
    assert!(output.contains("[key: string]: unknown;"));
    assert!(output.contains("metadata: UserMetadata"));
    assert!(!output.contains("metadata.user_id"));
}

#[test]
fn emits_thin_select_helpers_with_fetch() {
    let output = generate_fixture();
    assert!(output.contains("type SurrealLike,"));
    assert!(output.contains("export async function selectUser("));
    assert!(output.contains("export async function selectUserRelated("));
    assert!(output.contains("export async function selectPostRelated("));
    assert!(output.contains("export async function selectLikesRelated("));
    assert!(
        output.contains(
            "selectRecordRelated<UserSelected>(db, \"user\", id, [\"posts\", \"liked\"])"
        )
    );
    assert!(output.contains("selectRecordRelated<PostSelected>(db, \"post\", id, [\"author\"])"));
    assert!(
        output.contains("selectRecordRelated<LikesSelected>(db, \"likes\", id, [\"in\", \"out\"])")
    );
}

#[test]
fn emits_create_update_inputs_without_computed_fields() {
    let output = generate_fixture();
    let scalars_start = output.find("export type UserCreateScalars = {").expect("scalars");
    let create_start = output.find("export type UserCreateInput = ").expect("create");
    let scalars = &output[scalars_start..create_start];
    assert!(scalars.contains("email: string"));
    assert!(!scalars.contains("posts"));
    assert!(!scalars.contains("liked"));
    assert!(!scalars.contains("id:"));

    let create_end = output.find("export type UserUpdateInput = {").expect("update");
    let create = &output[create_start..create_end];
    assert!(create.contains("posts?: {"));
    assert!(create.contains("liked?: {"));
    assert!(create.contains("create?: Array<Omit<PostCreateScalars, \"author\">>"));

    let update_start = create_end;
    let update_end = output.find("export type PostCreateScalars").expect("PostCreateScalars");
    let update = &output[update_start..update_end];
    assert!(update.contains("email?: string"));
    assert!(update.contains("posts?: {"));

    let post_create_start = output.find("export type PostCreateInput = ").expect("PostCreate");
    let post_create_end = output.find("export type PostUpdateInput = {").expect("PostUpdate");
    let post_create = &output[post_create_start..post_create_end];
    assert!(post_create.contains("author:"));
    assert!(post_create.contains("connect: { id: string }"));
    assert!(post_create.contains("create: UserCreateScalars"));
}

#[test]
fn emits_as_record_id_and_crud_query_helpers() {
    let output = generate_fixture();
    assert!(output.contains("asRecordId,"));
    assert!(output.contains("selectRecord,"));
    assert!(output.contains("updateRecord,"));
    assert!(output.contains("deleteRecord,"));
    assert!(output.contains("queryRows,"));
    assert!(output.contains("findUniqueRecord,"));
    assert!(output.contains("findManyRecords,"));
    assert!(output.contains("export async function createUser("));
    assert!(output.contains("export async function updateUser("));
    assert!(output.contains("export async function deleteUser("));
    assert!(output.contains("return deleteRecord(db, \"user\", id)"));
    assert!(output.contains("export async function queryUsers("));
    assert!(output.contains("return queryRows<User>(db, sql, vars)"));
    assert!(output.contains("export async function queryUsersRelated("));
    assert!(output.contains("return queryRows<UserSelected>(db, sql, vars)"));
    assert!(output.contains("export async function createLikes("));
    assert!(output.contains("relateEdge,"));
    assert!(output.contains("relateEdge(db, \"likes\""));
}

#[test]
fn emits_get_payload_and_select_types() {
    let output = generate_fixture();
    assert!(output.contains("export type SelectArg<S, O = never>"));
    assert!(output.contains("export type ResolveSelectField<"));
    assert!(output.contains("export type UserScalars = {"));
    assert!(output.contains("export type UserSelect = {"));
    assert!(output.contains("email?: boolean"));
    assert!(output.contains("posts?: SelectArg<PostSelect, PostOrderByInput>"));
    assert!(output.contains("liked?: SelectArg<LikesSelect, LikesOrderByInput>"));
    assert!(output.contains("author?: SelectArg<UserSelect, UserOrderByInput>"));
    assert!(
        output.contains("export type UserGetPayload<S extends UserSelect | undefined = undefined>")
    );
    assert!(output.contains("PostGetPayload<N>[]"));
    assert!(output.contains("LikesGetPayload<N>[]"));
    assert!(output.contains("export type LikesSelect = {"));
    assert!(output.contains("in?: SelectArg<UserSelect, UserOrderByInput>"));
    assert!(output.contains("out?: SelectArg<PostSelect, PostOrderByInput>"));
    assert!(output.contains("export type LikesGetPayload"));
}

#[test]
fn emits_select_meta_and_projection_builder() {
    let output = generate_fixture();
    assert!(output.contains("export type FieldSelectMeta"));
    assert!(output.contains("export const SelectMetaByTable"));
    assert!(output.contains("kind: \"computed\""));
    assert!(output.contains("kind: \"stored\""));
    assert!(output.contains("kind: \"edge\""));
    assert!(output.contains("edgeTable: \"likes\""));
    assert!(output.contains("dir: \"out\""));
    assert!(output.contains("bindSchemaMeta(SelectMetaByTable, WriteMetaByTable)"));
}

#[test]
fn emits_nested_relation_order_by() {
    let output = generate_fixture();
    assert!(output.contains("orderBy?: [O] extends [never] ? never : O | O[]"));
    assert!(output.contains("posts?: SelectArg<PostSelect, PostOrderByInput>"));
    assert!(output.contains("liked?: SelectArg<LikesSelect, LikesOrderByInput>"));
    assert!(output.contains("take?: number"));
    assert!(output.contains("skip?: number"));
}

#[test]
fn emits_where_inputs_and_build_where() {
    let output = generate_fixture();
    assert!(output.contains("export type StringFilter = {"));
    assert!(output.contains("export type NumberFilter = {"));
    assert!(output.contains("export type ListRelationFilter<W> = {"));
    assert!(output.contains("some?: W;"));
    assert!(output.contains("every?: W;"));
    assert!(output.contains("none?: W;"));
    assert!(output.contains("export type RelationFilter<W> = {"));
    assert!(output.contains("export type UserWhereInput = {"));
    assert!(output.contains("email?: string | StringFilter"));
    assert!(output.contains("posts?: PostWhereInput | ListRelationFilter<PostWhereInput>"));
    assert!(output.contains("liked?: LikesWhereInput | ListRelationFilter<LikesWhereInput>"));
    assert!(output.contains("author?: UserWhereInput | RelationFilter<UserWhereInput>"));
    assert!(output.contains("export type LikesWhereInput = {"));
    assert!(output.contains("score?: number | NumberFilter"));
    assert!(output.contains("filter: \"string\""));
    assert!(output.contains("where?: Record<string, unknown>"));
}

#[test]
fn emits_find_unique_hybrid_where() {
    let output = generate_fixture();
    assert!(output.contains("where: UserWhereInput"));
    assert!(output.contains("where: LikesWhereInput"));
    assert!(output.contains(
        "findUniqueRecord(db, \"user\", args as { where: Record<string, unknown>; select?: Record<string, unknown>; vars?: Record<string, unknown> })"
    ));
}

#[test]
fn emits_order_by_take_skip() {
    let output = generate_fixture();
    assert!(output.contains("export type SortOrder = \"asc\" | \"desc\""));
    assert!(output.contains("export type OrderByRelationCount = { _count?: SortOrder }"));
    assert!(output.contains("export type UserOrderByInput = {"));
    assert!(output.contains("email?: SortOrder"));
    assert!(output.contains("age?: SortOrder"));
    assert!(output.contains("posts?: OrderByRelationCount"));
    assert!(output.contains("liked?: OrderByRelationCount"));
    assert!(!output.contains("author?: OrderByRelationCount"));
    assert!(output.contains("export type LikesOrderByInput = {"));
    assert!(output.contains("score?: SortOrder"));
    assert!(output.contains("orderBy?: UserOrderByInput | UserOrderByInput[]"));
    assert!(output.contains("take?: number"));
    assert!(output.contains("skip?: number"));
}

#[test]
fn emits_fluent_create_client() {
    let output = generate_fixture();
    assert!(output.contains("export function createClient(db: SurrealLike)"));
    assert!(output.contains("user: {"));
    assert!(output.contains("findUnique: <S extends UserSelect"));
    assert!(output.contains("findMany: <S extends UserSelect"));
    assert!(output.contains("select?: S;"));
    assert!(output.contains("where?: UserWhereInput"));
    assert!(output.contains("where: UserWhereInput"));
    assert!(output.contains("orderBy?: UserOrderByInput | UserOrderByInput[]"));
    assert!(output.contains("findUniqueRecord(db, \"user\""));
    assert!(output.contains("findManyRecords(db, \"user\""));
    assert!(output.contains("createUser(db, data)"));
}

#[test]
fn emits_transaction_client() {
    let output = generate_fixture();
    assert!(output.contains("type SurrealOpsLike,"));
    assert!(output.contains("type SurrealTransactionLike,"));
    assert!(output.contains("async function runTransaction<T>("));
    assert!(output.contains("function buildClientDelegates(db: SurrealOpsLike)"));
    assert!(
        output.contains("export type TransactionClient = ReturnType<typeof buildClientDelegates>")
    );
    assert!(
        output.contains(
            "$transaction: <T>(fn: (tx: TransactionClient) => Promise<T>): Promise<T> =>"
        )
    );
    assert!(output.contains("runTransaction(db, fn)"));
    assert!(output.contains("await txn.commit()"));
    assert!(output.contains("await txn.cancel()"));
}

#[test]
fn emits_raw_query_on_client() {
    let output = generate_fixture();
    assert!(output.contains(
        "$queryRaw: <T = unknown>(sql: string, vars?: Record<string, unknown>): Promise<T[]> =>"
    ));
    assert!(output.contains("      queryRows<T>(db, sql, vars),"));
    assert!(output.contains(
        "$executeRaw: (sql: string, vars?: Record<string, unknown>): Promise<unknown> =>"
    ));
    assert!(output.contains("      db.query(sql, vars),"));
}

#[test]
fn emits_live_select_on_session_client() {
    let output = generate_fixture();
    assert!(output.contains("type LiveAction,"));
    assert!(output.contains("type LiveHandle,"));
    assert!(output.contains("async function openLiveSelect<T>("));
    assert!(output.contains("throw new Error(\"$live: sql must be a LIVE SELECT statement\")"));
    assert!(output.contains("live: (): Promise<LiveHandle<User>> => db.live<User>(\"user\")"));
    assert!(output.contains("live: (): Promise<LiveHandle<Likes>> => db.live<Likes>(\"likes\")"));
    assert!(output.contains(
        "$live: <T = unknown>(sql: string, vars?: Record<string, unknown>): Promise<LiveHandle<T>> =>"
    ));
    assert!(output.contains("openLiveSelect<T>(db, sql, vars)"));

    let delegates_start = output.find("function buildClientDelegates").expect("delegates");
    let create_start = output.find("export function createClient").expect("createClient");
    let delegates = &output[delegates_start..create_start];
    assert!(!delegates.contains("LiveHandle"));
}

#[test]
fn emits_where_unique_and_bulk_upsert() {
    let output = generate_fixture();
    assert!(output.contains("export type UserWhereUniqueInput = {"));
    assert!(output.contains("email?: string | StringFilter"));
    assert!(output.contains("export type LikesWhereUniqueInput = {"));
    assert!(output.contains("id?: string | IdFilter"));
    assert!(output.contains("createManyVia,"));
    assert!(output.contains("updateManyRecords,"));
    assert!(output.contains("deleteManyRecords,"));
    assert!(output.contains("upsertRecord,"));
    assert!(output.contains("createMany: <S extends UserSelect"));
    assert!(output.contains("updateMany: <S extends UserSelect"));
    assert!(output.contains("deleteMany: <S extends UserSelect"));
    assert!(output.contains("upsert: <S extends UserSelect"));
    assert!(output.contains("where: UserWhereUniqueInput"));
    assert!(output.contains("ManyReturnResult<S, R, User, UserGetPayload<S>>"));
    assert!(output.contains("createManyVia(db, \"user\""));
    assert!(output.contains("updateManyRecords(db, \"user\""));
    assert!(output.contains("deleteManyRecords(db, \"user\""));
    assert!(output.contains("upsertRecord(db, \"user\""));
    assert!(output.contains("createManyVia(db, \"likes\""));
    assert!(output.contains("deleteManyRecords(db, \"likes\""));
}

#[test]
fn emits_nested_writes() {
    let output = generate_fixture();
    assert!(output.contains("export type FieldWriteMeta ="));
    assert!(output.contains("export const WriteMetaByTable"));
    assert!(output.contains("backLinkField: \"author\""));
    assert!(output.contains("kind: \"edge\""));
    assert!(output.contains("createWithNested,"));
    assert!(output.contains("updateWithNested,"));
    assert!(output.contains("relateEdge,"));
    assert!(output.contains("createWithNested<User>(db, \"user\""));
    assert!(output.contains("updateWithNested(db, \"user\""));
}

#[test]
fn emits_count_and_groupby() {
    let output = generate_fixture();
    assert!(output.contains("export type UserScalarFieldEnum ="));
    assert!(output.contains("export type UserNumericFieldEnum = \"age\""));
    assert!(output.contains("export type UserGroupByOrderByInput = {"));
    assert!(output.contains("export type LikesScalarFieldEnum ="));
    assert!(output.contains("export type LikesNumericFieldEnum = \"score\""));
    assert!(output.contains("countRecords,"));
    assert!(output.contains("groupByRecords,"));
    assert!(output.contains("having?: UserHavingInput"));
    assert!(output.contains("having?: LikesHavingInput"));
    assert!(output.contains("countRecords(db, \"user\""));
    assert!(output.contains("groupByRecords(db, \"user\""));
    assert!(output.contains("countRecords(db, \"likes\""));
    assert!(output.contains("groupByRecords(db, \"likes\""));
    assert!(output.contains("count: (args: { where?: UserWhereInput } = {}): Promise<number>"));
    assert!(output.contains("by: UserScalarFieldEnum[]"));
}

#[test]
fn emits_single_update_delete_opts() {
    let output = generate_fixture();
    assert!(output.contains("type SingleUpdateResult,"));
    assert!(output.contains("type SingleDeleteResult,"));
    assert!(output.contains("type SingleUpsertResult,"));
    assert!(output.contains("updateOneRecord,"));
    assert!(output.contains("deleteOneRecord,"));
    assert!(output.contains("opts?: { select?: S; return?: R }"));
    assert!(output.contains("SingleUpdateResult<S, R, User, UserGetPayload<S>>"));
    assert!(output.contains("SingleDeleteResult<S, R, User, UserGetPayload<S>>"));
    assert!(output.contains("SingleUpsertResult<S, R, User, UserGetPayload<S>>"));
    assert!(output.contains("return?: MutationReturn"));
}

#[test]
fn emits_mutation_return_modes() {
    let output = generate_fixture();
    assert!(output.contains("type MutationReturn,"));
    assert!(output.contains("type ManyReturnResult,"));
    assert!(output.contains("return?: MutationReturn"));
    assert!(output.contains("createUser(db, data as UserCreateInput), false)"));
    assert!(output.contains("createLikes(db, data as LikesCreateInput), true)"));
    assert!(output.contains("Exclude<MutationReturn, \"BEFORE\">"));
    assert!(output.contains("Exclude<MutationReturn, \"AFTER\">"));
    assert!(output.contains("return?: R"));
}

#[test]
fn maps_scalar_field_types() {
    let schema = parse(
        r#"
datasource db { provider = "surrealdb" }
model Sample {
  id @id
  flag bool
  count int
  ratio float
  label string
  when datetime
  tags string[]
  blob object
}
"#,
    )
    .expect("parse");
    let output = CodeGenerator::generate(&TypeScriptGenerator::new(), &schema).expect("generate");
    assert!(output.contains("flag: boolean"));
    assert!(output.contains("count: number"));
    assert!(output.contains("ratio: number"));
    assert!(output.contains("label: string"));
    assert!(output.contains("when: Date"));
    assert!(output.contains("tags: string[]"));
    assert!(output.contains("blob: Record<string, unknown>"));
    assert!(output.contains("id: RecordId<\"sample\">"));
}

#[test]
fn maps_custom_field_type_passthrough() {
    use std::collections::BTreeMap;

    use core::{DatabaseSchema, Datasource, Field, Model, NamingConvention, TableMode};

    let schema = DatabaseSchema {
        datasource: Datasource {
            provider: "surrealdb".to_owned(),
            url: None,
            namespace: None,
            database: None,
            extra: BTreeMap::new(),
        },
        naming: NamingConvention { tables: NamingCase::SnakeCase, fields: None },
        generators: Vec::new(),
        object_types: Vec::new(),
        models: vec![Model {
            name: "Sample".to_owned(),
            fields: vec![Field {
                name: "custom".to_owned(),
                field_type: FieldType::Custom("GeoPoint".to_owned()),
                optional: false,
                unique: false,
                is_id: false,
                default_value: None,
                default_always: false,
                value_expression: None,
                readonly: false,
                flexible: false,
                link_target: None,
                link_name: None,
                on_delete: None,
                link_storage: None,
                link_opposite_field: None,
                relation_name: None,
                attributes: BTreeMap::new(),
            }],
            table_mode: TableMode::Schemafull,
            permissions: None,
            indexes: Vec::new(),
            attributes: BTreeMap::new(),
        }],
        edges: Vec::new(),
    };
    let output = CodeGenerator::generate(&TypeScriptGenerator::new(), &schema).expect("generate");
    assert!(output.contains("custom: GeoPoint"));
}

#[test]
fn respects_optional_marker() {
    use std::collections::BTreeMap;

    use core::{DatabaseSchema, Datasource, Field, Model, NamingConvention, TableMode};

    let schema = DatabaseSchema {
        datasource: Datasource {
            provider: "surrealdb".to_owned(),
            url: None,
            namespace: None,
            database: None,
            extra: BTreeMap::new(),
        },
        naming: NamingConvention { tables: NamingCase::SnakeCase, fields: None },
        generators: Vec::new(),
        object_types: Vec::new(),
        models: vec![Model {
            name: "Sample".to_owned(),
            fields: vec![Field {
                name: "value".to_owned(),
                field_type: FieldType::String,
                optional: true,
                unique: false,
                is_id: false,
                default_value: None,
                default_always: false,
                value_expression: None,
                readonly: false,
                flexible: false,
                link_target: None,
                link_name: None,
                on_delete: None,
                link_storage: None,
                link_opposite_field: None,
                relation_name: None,
                attributes: BTreeMap::new(),
            }],
            table_mode: TableMode::Schemafull,
            permissions: None,
            indexes: Vec::new(),
            attributes: BTreeMap::new(),
        }],
        edges: Vec::new(),
    };
    let output = CodeGenerator::generate(&TypeScriptGenerator::new(), &schema).expect("generate");
    assert!(output.contains("value?: string"));
}
