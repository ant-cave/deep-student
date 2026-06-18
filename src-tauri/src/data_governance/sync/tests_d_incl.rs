                preview_json TEXT
            );
            CREATE TABLE exam_sheets (
                id TEXT PRIMARY KEY,
                preview_json TEXT
            );
            CREATE TABLE questions (
                id TEXT PRIMARY KEY,
                images_json TEXT
            );
            "#,
        )
        .unwrap();

        // 6 个 blob：主文件、PDF 页图、压缩页图、试卷页图、题目图片、
        // "压缩不划算"去重 blob。初始 ref_count 全部写成错误值，验证重算后恢复正确。
        for hash in [
            "main-blob",
            "page-blob",
            "page-compressed-blob",
            "exam-page-blob",
            "question-img-blob",
            "dedup-blob",
        ] {
            conn.execute(
                "INSERT INTO blobs(hash, ref_count) VALUES (?1, 99)",
                rusqlite::params![hash],
            )
            .unwrap();
        }

        conn.execute(
            r#"INSERT INTO files(id, blob_hash, compressed_blob_hash, preview_json) VALUES (
                'file-1', 'main-blob', NULL,
                '{"pages":[{"page_index":0,"blob_hash":"page-blob","compressed_blob_hash":"page-compressed-blob"}]}'
            )"#,
            [],
        )
        .unwrap();
        // "压缩不划算"：文件级与页级 compressed 均指向原始 blob，
        // 本地只 +1（store 一次），重算必须按等值排除避免双计。
        conn.execute(
            r#"INSERT INTO files(id, blob_hash, compressed_blob_hash, preview_json) VALUES (
                'file-2', 'dedup-blob', 'dedup-blob',
                '{"pages":[{"page_index":0,"blob_hash":"dedup-blob","compressed_blob_hash":"dedup-blob"}]}'
            )"#,
            [],
        )
        .unwrap();
        conn.execute(
            r#"INSERT INTO exam_sheets(id, preview_json) VALUES (
                'exam-1', '{"pages":[{"page_index":0,"blob_hash":"exam-page-blob"}]}'
            )"#,
            [],
        )
        .unwrap();
        // 两道题引用同一张图：引用计数应为出现次数 2
        conn.execute(
            r#"INSERT INTO questions(id, images_json) VALUES
                ('q-1', '[{"id":"att_1","hash":"question-img-blob"}]'),
                ('q-2', '[{"id":"att_2","hash":"question-img-blob"}]')"#,
            [],
        )
        .unwrap();
        // 非法 JSON 行不得让重算报错（按无引用处理）
        conn.execute(
            "INSERT INTO files(id, blob_hash, preview_json) VALUES ('file-bad', NULL, '{not json')",
            [],
        )
        .unwrap();

        SyncManager::recompute_blob_ref_counts(&conn).unwrap();

        let count_of = |hash: &str| -> i64 {
            conn.query_row(
                "SELECT ref_count FROM blobs WHERE hash = ?1",
                rusqlite::params![hash],
                |row| row.get(0),
            )
            .unwrap()
        };
        assert_eq!(count_of("main-blob"), 1, "显式列引用");
        assert_eq!(count_of("page-blob"), 1, "preview_json 页图引用不得清零");
        assert_eq!(
            count_of("page-compressed-blob"),
            1,
            "preview_json 压缩页图引用不得清零"
        );
        assert_eq!(
            count_of("exam-page-blob"),
            1,
            "exam_sheets.preview_json 页图引用不得清零"
        );
        assert_eq!(
            count_of("question-img-blob"),
            2,
            "questions.images_json 按出现次数计数"
        );
        // file-2：files.blob_hash(+1) + 页图 blob_hash(+1)；
        // 文件级/页级 compressed 与原始等值 → 不重复计数（本地未额外 store）
        assert_eq!(
            count_of("dedup-blob"),
            2,
            "压缩不划算（compressed==原始）不得双计"
        );
    }

    /// JSON 引用表（questions/exam_sheets）变更也必须触发重算门控
    #[test]
    fn regression_changes_to_json_ref_tables_trigger_recompute_gate() {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(
            r#"
            CREATE TABLE questions (id TEXT PRIMARY KEY, images_json TEXT);
            CREATE TABLE exam_sheets (id TEXT PRIMARY KEY, preview_json TEXT);
            CREATE TABLE notes (id TEXT PRIMARY KEY, title TEXT);
            "#,
        )
        .unwrap();

        let change_for = |table: &str| SyncChangeWithData {
            table_name: table.to_string(),
            record_id: "r1".to_string(),
            operation: ChangeOperation::Update,
            data: None,
            changed_at: "2026-06-12T00:00:00Z".to_string(),
            change_log_id: Some(1),
            database_name: Some("vfs".to_string()),
            suppress_change_log: Some(true),
            source_device_id: None,
            source_seq: None,
        };

        assert!(SyncManager::changes_affect_ref_counts(
            &conn,
            &[change_for("questions")]
        ));
        assert!(SyncManager::changes_affect_ref_counts(
            &conn,
            &[change_for("exam_sheets")]
        ));
        assert!(!SyncManager::changes_affect_ref_counts(
            &conn,
            &[change_for("notes")]
        ));
    }

    #[test]
    fn test_conflict_guard_orders_blob_metadata_before_files() {
        let conn = create_vfs_blob_fk_test_db();
        let changes = vec![blob_file_change(), blob_metadata_change()];

        let (result, conflict_result) = SyncManager::apply_downloaded_changes_with_conflict_guard(
            &conn,
            &changes,
            None,
            conflict_resolver::ConflictPolicy::KeepLatest,
            Some("cloud-device"),
            Some("local-device"),
        )
        .unwrap();

        assert_eq!(result.success_count, 2);
        assert_eq!(conflict_result.conflicts_saved, 0);
        assert_blob_file_applied(&conn);
    }

    #[test]
    fn regression_p3_5_files_same_sha_is_aliased_to_existing_file() {
        let conn = create_vfs_blob_fk_test_db();
        conn.execute(
            "INSERT INTO blobs(hash, relative_path, size, mime_type, ref_count, created_at)
             VALUES(?1, ?2, ?3, ?4, ?5, ?6)",
            rusqlite::params![
                "blob-hash-1",
                "bl/ob/blob-hash-1.md",
                12i64,
                "text/markdown",
                1i64,
                1780225200000i64
            ],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO files(id, blob_hash, sha256, file_name, size, created_at, updated_at)
             VALUES(?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            rusqlite::params![
                "file-1",
                "blob-hash-1",
                "same-sha",
                "original.md",
                12i64,
                "2026-05-31T00:00:00Z",
                "2026-05-31T00:00:00Z"
            ],
        )
        .unwrap();

        let changes = vec![SyncChangeWithData {
            table_name: "files".to_string(),
            record_id: "file-2".to_string(),
            operation: ChangeOperation::Insert,
            data: Some(serde_json::json!({
                "id": "file-2",
                "blob_hash": "blob-hash-1",
                "sha256": "same-sha",
                "file_name": "must-not-merge.md",
                "size": 12,
                "created_at": "2026-06-01T00:00:00Z",
                "updated_at": "2026-06-01T00:00:00Z"
            })),
            changed_at: "2026-06-01T00:00:00Z".to_string(),
            change_log_id: Some(42),
            database_name: Some("vfs".to_string()),
            suppress_change_log: Some(true),
            source_device_id: None,
            source_seq: None,
        }];

        let result = SyncManager::apply_downloaded_changes(&conn, &changes, None).unwrap();

        assert_eq!(result.success_count, 1);
        assert_eq!(result.failure_count, 0);

        let original_name: String = conn
            .query_row("SELECT file_name FROM files WHERE id='file-1'", [], |row| {
                row.get(0)
            })
            .unwrap();
        let file_2_count: i64 = conn
            .query_row("SELECT COUNT(*) FROM files WHERE id='file-2'", [], |row| {
                row.get(0)
            })
            .unwrap();
        let quarantine_count: i64 = conn
            .query_row("SELECT COUNT(*) FROM __sync_quarantine", [], |row| {
                row.get(0)
            })
            .unwrap();
        let alias_canonical: String = conn
            .query_row(
                "SELECT canonical_id FROM __sync_id_aliases
                 WHERE table_name='files' AND remote_id='file-2'",
                [],
                |row| row.get(0),
            )
            .unwrap();

        assert_eq!(original_name, "must-not-merge.md");
        assert_eq!(file_2_count, 0);
        assert_eq!(quarantine_count, 0);
        assert_eq!(alias_canonical, "file-1");
    }

    #[test]
    fn regression_m15_sqlite_text_json_string_is_not_reserialized() {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(
            r#"
            CREATE TABLE docs (
                id TEXT PRIMARY KEY,
                body TEXT
            );
            INSERT INTO docs (id, body)
            VALUES ('d1', '{"b":2, "a":1}');
            "#,
        )
        .unwrap();

        let body = conn
            .query_row("SELECT body FROM docs WHERE id='d1'", [], |row| {
                Ok(SyncManager::sqlite_value_to_json(row, 0))
            })
            .unwrap();

        assert_eq!(body, serde_json::json!("{\"b\":2, \"a\":1}"));
    }

    #[test]
    fn regression_m15_sqlite_blob_is_typed_dsblob_payload() {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(
            r#"
            CREATE TABLE docs (
                id TEXT PRIMARY KEY,
                payload BLOB
            );
            INSERT INTO docs (id, payload)
            VALUES ('d1', x'000102ff');
            "#,
        )
        .unwrap();

        let payload = conn
            .query_row("SELECT payload FROM docs WHERE id='d1'", [], |row| {
                Ok(SyncManager::sqlite_value_to_json(row, 0))
            })
            .unwrap();

        assert_eq!(payload, serde_json::json!({ "$dsblob": "AAEC/w==" }));
        let param = SyncManager::json_value_to_sql_param(&payload).unwrap();
        let restored: Vec<u8> = conn
            .query_row("SELECT ?1", [&param.as_ref()], |row| row.get(0))
            .unwrap();
        assert_eq!(restored, vec![0, 1, 2, 255]);
    }

    fn create_resource_alias_test_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(
            r#"
            PRAGMA foreign_keys = ON;
            CREATE TABLE resources (
                id TEXT PRIMARY KEY,
                hash TEXT NOT NULL UNIQUE,
                body TEXT,
                updated_at TEXT
            );
            CREATE TABLE resource_notes (
                id TEXT PRIMARY KEY,
                resource_id TEXT NOT NULL,
                note TEXT,
                updated_at TEXT,
                FOREIGN KEY(resource_id) REFERENCES resources(id)
            );
            INSERT INTO resources (id, hash, body, updated_at)
            VALUES ('local-res', 'same-business-hash', 'local body', '2024-01-01T00:00:00Z');
            "#,
        )
        .unwrap();
        conn
    }

    fn resource_alias_parent_change() -> SyncChangeWithData {
        SyncChangeWithData {
            table_name: "resources".to_string(),
            record_id: "remote-res".to_string(),
            operation: ChangeOperation::Insert,
            data: Some(serde_json::json!({
                "id": "remote-res",
                "hash": "same-business-hash",
                "body": "cloud body",
                "updated_at": "2024-01-02T00:00:00Z"
            })),
            changed_at: "2024-01-02T00:00:00Z".to_string(),
            change_log_id: None,
            database_name: Some("vfs".to_string()),
            suppress_change_log: None,
            source_device_id: None,
            source_seq: None,
        }
    }

    fn resource_alias_child_change() -> SyncChangeWithData {
        SyncChangeWithData {
            table_name: "resource_notes".to_string(),
            record_id: "note-remote".to_string(),
            operation: ChangeOperation::Insert,
            data: Some(serde_json::json!({
                "id": "note-remote",
                "resource_id": "remote-res",
                "note": "child uses remote id",
                "updated_at": "2024-01-02T00:00:01Z"
            })),
            changed_at: "2024-01-02T00:00:01Z".to_string(),
            change_log_id: None,
            database_name: Some("vfs".to_string()),
            suppress_change_log: None,
            source_device_id: None,
            source_seq: None,
        }
    }

    fn assert_resource_alias_result(conn: &Connection) {
        let resource_count: i64 = conn
            .query_row("SELECT COUNT(*) FROM resources", [], |row| row.get(0))
            .unwrap();
        assert_eq!(
            resource_count, 1,
            "business-key conflict should reuse local row"
        );

        let remote_count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM resources WHERE id = 'remote-res'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(
            remote_count, 0,
            "remote id should be an alias, not a new row"
        );

        let body: String = conn
            .query_row(
                "SELECT body FROM resources WHERE id = 'local-res'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(body, "cloud body");

        let child_fk: String = conn
            .query_row(
                "SELECT resource_id FROM resource_notes WHERE id = 'note-remote'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(child_fk, "local-res", "child FK should be remapped");

        let violations = SyncManager::collect_foreign_key_violations(conn, 20).unwrap();
        assert!(
            violations.is_empty(),
            "foreign keys should pass: {:?}",
            violations
        );
    }

    #[test]
    fn test_business_key_alias_remaps_child_fk_when_child_arrives_first() {
        let conn = create_resource_alias_test_db();
        let changes = vec![
            resource_alias_child_change(),
            resource_alias_parent_change(),
        ];

        let result = SyncManager::apply_downloaded_changes(&conn, &changes, None).unwrap();

        assert_eq!(result.success_count, 2);
        assert_resource_alias_result(&conn);
    }

    #[test]
    fn test_business_key_alias_reuses_canonical_id_when_parent_arrives_first() {
        let conn = create_resource_alias_test_db();
        let changes = vec![
            resource_alias_parent_change(),
            resource_alias_child_change(),
        ];

        let result = SyncManager::apply_downloaded_changes(&conn, &changes, None).unwrap();

        assert_eq!(result.success_count, 2);
        assert_resource_alias_result(&conn);
    }

    #[test]
    fn regression_m9_same_batch_business_key_alias_remaps_later_child_fk() {
        let conn = create_resource_alias_test_db();
        conn.execute("DELETE FROM resource_notes", []).unwrap();
        conn.execute("DELETE FROM resources", []).unwrap();

        let parent_a = SyncChangeWithData {
            table_name: "resources".to_string(),
            record_id: "remote-res-a".to_string(),
            operation: ChangeOperation::Insert,
            data: Some(serde_json::json!({
                "id": "remote-res-a",
                "hash": "same-batch-hash",
                "body": "first parent",
                "updated_at": "2024-01-02T00:00:00Z"
            })),
            changed_at: "2024-01-02T00:00:00Z".to_string(),
            change_log_id: None,
            database_name: Some("vfs".to_string()),
            suppress_change_log: None,
            source_device_id: None,
            source_seq: None,
        };
        let parent_b = SyncChangeWithData {
            table_name: "resources".to_string(),
            record_id: "remote-res-b".to_string(),
            operation: ChangeOperation::Insert,
            data: Some(serde_json::json!({
                "id": "remote-res-b",
                "hash": "same-batch-hash",
                "body": "second parent",
                "updated_at": "2024-01-02T00:00:01Z"
            })),
            changed_at: "2024-01-02T00:00:01Z".to_string(),
            change_log_id: None,
            database_name: Some("vfs".to_string()),
            suppress_change_log: None,
            source_device_id: None,
            source_seq: None,
        };
        let child = SyncChangeWithData {
            table_name: "resource_notes".to_string(),
            record_id: "note-remote-b".to_string(),
            operation: ChangeOperation::Insert,
            data: Some(serde_json::json!({
                "id": "note-remote-b",
                "resource_id": "remote-res-b",
                "note": "child references duplicate parent",
                "updated_at": "2024-01-02T00:00:02Z"
            })),
            changed_at: "2024-01-02T00:00:02Z".to_string(),
            change_log_id: None,
            database_name: Some("vfs".to_string()),
            suppress_change_log: None,
            source_device_id: None,
            source_seq: None,
        };

        let result =
            SyncManager::apply_downloaded_changes(&conn, &[parent_a, parent_b, child], None)
                .unwrap();

        assert_eq!(result.success_count, 3);
        assert_eq!(result.failure_count, 0);
        let resource_count: i64 = conn
            .query_row("SELECT COUNT(*) FROM resources", [], |row| row.get(0))
            .unwrap();
        let child_fk: String = conn
            .query_row(
                "SELECT resource_id FROM resource_notes WHERE id='note-remote-b'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        let alias_canonical: String = conn
            .query_row(
                "SELECT canonical_id FROM __sync_id_aliases
                 WHERE table_name='resources' AND remote_id='remote-res-b'",
                [],
                |row| row.get(0),
            )
            .unwrap();

        assert_eq!(resource_count, 1);
        assert_eq!(child_fk, "remote-res-a");
        assert_eq!(alias_canonical, "remote-res-a");
        assert!(SyncManager::collect_foreign_key_violations(&conn, 20)
            .unwrap()
            .is_empty());
    }

    #[test]
    fn test_conflict_guard_business_key_alias_remaps_child_fk() {
        let conn = create_resource_alias_test_db();
        let changes = vec![
            resource_alias_child_change(),
            resource_alias_parent_change(),
        ];

        let (result, conflict_result) = SyncManager::apply_downloaded_changes_with_conflict_guard(
            &conn,
            &changes,
            None,
            conflict_resolver::ConflictPolicy::KeepLatest,
            Some("cloud-device"),
            Some("local-device"),
        )
        .unwrap();

        assert_eq!(result.success_count, 2);
        assert_eq!(conflict_result.conflicts_saved, 0);
        assert_resource_alias_result(&conn);
    }

    #[test]
    fn test_suppress_change_log_does_not_mark_existing_user_update() {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(
            r#"
            CREATE TABLE test_records (
                id TEXT PRIMARY KEY,
                content TEXT,
                updated_at TEXT
            );
            CREATE TABLE __change_log (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                table_name TEXT NOT NULL,
                record_id TEXT NOT NULL,
                operation TEXT NOT NULL,
                changed_at TEXT NOT NULL DEFAULT (datetime('now')),
                sync_version INTEGER DEFAULT 0
            );
            CREATE TRIGGER trg_echo_insert
            AFTER INSERT ON test_records
            BEGIN
                INSERT INTO __change_log(table_name, record_id, operation)
                VALUES('test_records', NEW.id, 'INSERT');
            END;
            CREATE TRIGGER trg_echo_update
            AFTER UPDATE ON test_records
            BEGIN
                INSERT INTO __change_log(table_name, record_id, operation)
                VALUES('test_records', NEW.id, 'UPDATE');
            END;
            "#,
        )
        .unwrap();

        // 首次云端回放：应只抑制回放引入的 echo 记录
        let replay_insert = vec![SyncChangeWithData {
            table_name: "test_records".to_string(),
            record_id: "r1".to_string(),
            operation: ChangeOperation::Insert,
            data: Some(serde_json::json!({
                "id": "r1",
                "content": "cloud",
                "updated_at": "2026-02-10T00:00:00Z"
            })),
            changed_at: "2026-02-10T00:00:00Z".to_string(),
            change_log_id: None,
            database_name: None,
            suppress_change_log: Some(true),
            source_device_id: None,
            source_seq: None,
        }];
        SyncManager::apply_downloaded_changes(&conn, &replay_insert, None).unwrap();

        // 本地用户编辑，产生 UPDATE 日志（应该保持未同步）
        conn.execute(
            "UPDATE test_records SET content = 'local-edit' WHERE id = 'r1'",
            [],
        )
        .unwrap();
        let user_update_log_id: i64 = conn
            .query_row(
                "SELECT id FROM __change_log WHERE operation = 'UPDATE' ORDER BY id DESC LIMIT 1",
                [],
                |row| row.get(0),
            )
            .unwrap();

        // 再次回放同一个 INSERT，验证不会误标记用户 UPDATE 记录
        SyncManager::apply_downloaded_changes(&conn, &replay_insert, None).unwrap();

        let user_sync_version: i64 = conn
            .query_row(
                "SELECT sync_version FROM __change_log WHERE id = ?1",
                params![user_update_log_id],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(
            user_sync_version, 0,
            "existing user update log must not be marked as synced by replay suppression"
        );
    }
}
