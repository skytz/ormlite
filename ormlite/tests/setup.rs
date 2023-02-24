#[allow(dead_code)]
pub fn migrate_self(files: &[&str]) -> sqlmo::Migration {
    use ormlite_core::schema::TryFromOrmlite;
    let paths = files.iter().map(std::path::Path::new).collect::<Vec<_>>();
    let options = ormlite_core::schema::Options { verbose: false };
    let schema: sqlmo::Schema = TryFromOrmlite::try_from_ormlite_project(&paths, &options).unwrap();
    let migration = sqlmo::Schema::default().migrate_to(schema, &sqlmo::MigrationOptions::default()).unwrap();
    migration
}