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
fn default_generator_is_constructible() {
    assert_eq!(
        CodeGenerator::language(&TypeScriptGenerator::new()),
        "typescript"
    );
}

#[test]
fn emits_record_id_helper_and_tables() {
    let output = generate_fixture();
    assert!(output.contains("export type RecordId<Table extends string = string>"));
    assert!(output.contains("export function recordId<Table extends string>"));
    assert!(output.contains("export function parseRecordId"));
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
    let selected_end = output
        .find("export type UserCreateInput = {")
        .expect("UserCreateInput");
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
    assert!(output.contains("export type SurrealLike"));
    assert!(output.contains("export async function selectUser("));
    assert!(output.contains("export async function selectUserRelated("));
    assert!(output.contains("export async function selectPostRelated("));
    assert!(output.contains("export async function selectLikesRelated("));
    assert!(output.contains("FETCH posts, liked"));
    assert!(output.contains("FETCH author"));
    assert!(output.contains("FETCH in, out"));
}

#[test]
fn emits_create_update_inputs_without_computed_fields() {
    let output = generate_fixture();
    let create_start = output.find("export type UserCreateInput = {").expect("create");
    let create_end = output.find("export type UserUpdateInput = {").expect("update");
    let create = &output[create_start..create_end];
    assert!(create.contains("email: string"));
    assert!(!create.contains("posts"));
    assert!(!create.contains("liked"));
    assert!(!create.contains("id:"));

    let update_start = create_end;
    let update_end = output.find("export type UserSelectFields").expect("select fields");
    let update = &output[update_start..update_end];
    assert!(update.contains("email?: string"));
    assert!(!update.contains("posts"));

    let post_create_start = output.find("export type PostCreateInput = {").expect("PostCreate");
    let post_create_end = output.find("export type PostUpdateInput = {").expect("PostUpdate");
    let post_create = &output[post_create_start..post_create_end];
    assert!(post_create.contains("author: RecordId<\"user\">"));
}

#[test]
fn emits_as_record_id_and_crud_query_helpers() {
    let output = generate_fixture();
    assert!(output.contains("export function asRecordId<Table extends string>"));
    assert!(output.contains("create<T = unknown>(thing: string"));
    assert!(output.contains("merge<T = unknown>(thing: string"));
    assert!(output.contains("delete<T = unknown>(thing: string"));
    assert!(output.contains("export async function createUser("));
    assert!(output.contains("export async function updateUser("));
    assert!(output.contains("export async function deleteUser("));
    assert!(output.contains("export async function queryUsers("));
    assert!(output.contains("export async function queryUsersRelated("));
    assert!(output.contains("export async function createLikes("));
    assert!(output.contains("RELATE $in->likes->$out"));
}

#[test]
fn emits_get_payload_and_select_types() {
    let output = generate_fixture();
    assert!(output.contains("export type UserSelectFields = {"));
    assert!(output.contains("posts: PostSelected[]"));
    assert!(output.contains("liked: LikesSelected[]"));
    assert!(output.contains("export type UserSelect ="));
    assert!(output.contains("export type UserGetPayload<S extends UserSelect | undefined = undefined>"));
    assert!(output.contains("export type LikesGetPayload"));
}

#[test]
fn emits_fluent_create_client() {
    let output = generate_fixture();
    assert!(output.contains("export function createClient(db: SurrealLike)"));
    assert!(output.contains("user: {"));
    assert!(output.contains("async findUnique<S extends UserSelect"));
    assert!(output.contains("async findMany<S extends UserSelect"));
    assert!(output.contains("select?: S;"));
    assert!(output.contains("FETCH ${keys.join(\", \")}"));
    assert!(output.contains("return createUser(db, data);"));
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
    let output =
        CodeGenerator::generate(&TypeScriptGenerator::new(), &schema).expect("generate");
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
    let output =
        CodeGenerator::generate(&TypeScriptGenerator::new(), &schema).expect("generate");
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
    let output =
        CodeGenerator::generate(&TypeScriptGenerator::new(), &schema).expect("generate");
    assert!(output.contains("value?: string"));
}
