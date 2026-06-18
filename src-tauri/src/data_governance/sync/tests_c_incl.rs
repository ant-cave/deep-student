            table_name: "test_records".to_string(),
            record_id: "r1".to_string(),
            operation: ChangeOperation::Insert,
            data: Some(serde_json::json!({
                "id": "r1",
                "content": "same",
                "updated_at": "2026-02-10T00:00:00Z"
            })),
            changed_at: "2026-02-10T00:00:00Z".to_string(),
            change_log_id: None,
            database_name: Some("vfs".to_string()),
            suppress_change_log: Some(true),
            source_device_id: None,
            source_seq: None,
        }];

        SyncManager::apply_downloaded_changes(&conn, &changes, None).unwrap();
        SyncManager::apply_downloaded_changes(&conn, &changes, None).unwrap();

        let log_count: i64 = conn
            .query_row("SELECT COUNT(*) FROM __change_log", [], |r| r.get(0))
            .unwrap();
        let unsynced: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM __change_log WHERE sync_version = 0",
                [],
                |r| r.get(0),
            )
            .unwrap();

        assert_eq!(
            log_count, 1,
            "equivalent replay must not generate another local echo log"
        );
        assert_eq!(unsynced, 0);
    }

    #[test]
    fn test_dedupe_downloaded_changes_collapses_equivalent_payloads() {
        let first = SyncChangeWithData {
            table_name: "test_records".to_string(),
            record_id: "r1".to_string(),
            operation: ChangeOperation::Update,
            data: Some(serde_json::json!({
                "id": "r1",
                "content": "same",
                "updated_at": "2026-02-10T00:00:00Z"
            })),
            changed_at: "2026-02-10T00:00:00Z".to_string(),
            change_log_id: Some(1),
            database_name: Some("vfs".to_string()),
            suppress_change_log: None,
            source_device_id: None,
            source_seq: None,
        };
        let mut duplicate = first.clone();
        duplicate.changed_at = "2026-02-10T00:00:05Z".to_string();
        duplicate.change_log_id = Some(99);

        let deduped = SyncManager::dedupe_downloaded_changes(vec![(1, first), (2, duplicate)]);

        assert_eq!(
            deduped.len(),
            1,
            "same final payload from multiple remote packages should apply once"
        );
    }

    #[test]
    fn test_detect_record_conflicts_with_diverged_sync_versions() {
        let local_records = vec![RecordSnapshot {
            table_name: "messages".to_string(),
            record_id: "msg-1".to_string(),
            local_version: 12,
            sync_version: 10,
            updated_at: "2026-02-10T10:00:00Z".to_string(),
            deleted_at: None,
            data: serde_json::json!({"content": "local edit"}),
        }];
        let cloud_records = vec![RecordSnapshot {
            table_name: "messages".to_string(),
            record_id: "msg-1".to_string(),
            local_version: 21,
            sync_version: 20,
            updated_at: "2026-02-10T10:01:00Z".to_string(),
            deleted_at: None,
            data: serde_json::json!({"content": "cloud edit"}),
        }];

        let conflicts =
            SyncManager::detect_record_conflicts("chat_v2", &local_records, &cloud_records);
        assert_eq!(
            conflicts.len(),
            1,
            "diverged sync_version should still detect conflict"
        );
    }

    #[test]
    fn test_detect_record_conflicts_same_data_not_conflict() {
        let local_records = vec![RecordSnapshot {
            table_name: "messages".to_string(),
            record_id: "msg-1".to_string(),
            local_version: 12,
            sync_version: 10,
            updated_at: "2026-02-10T10:00:00Z".to_string(),
            deleted_at: None,
            data: serde_json::json!({"content": "same"}),
        }];
        let cloud_records = vec![RecordSnapshot {
            table_name: "messages".to_string(),
            record_id: "msg-1".to_string(),
            local_version: 21,
            sync_version: 20,
            updated_at: "2026-02-10T10:01:00Z".to_string(),
            deleted_at: None,
            data: serde_json::json!({"content": "same"}),
        }];

        let conflicts =
            SyncManager::detect_record_conflicts("chat_v2", &local_records, &cloud_records);
        assert!(
            conflicts.is_empty(),
            "same payload should not be treated as conflict even when both modified"
        );
    }

    #[test]
    fn test_apply_delete_uses_tombstone_when_column_exists() {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(
            r#"
            CREATE TABLE test_records (
                id TEXT PRIMARY KEY,
                content TEXT,
                deleted_at TEXT
            );
            INSERT INTO test_records (id, content, deleted_at)
            VALUES ('r1', 'alive', NULL);
            "#,
        )
        .unwrap();

        let changes = vec![SyncChangeWithData {
            table_name: "test_records".to_string(),
            record_id: "r1".to_string(),
            operation: ChangeOperation::Delete,
            data: None,
            changed_at: "2026-02-10T00:00:00Z".to_string(),
            change_log_id: None,
            database_name: None,
            suppress_change_log: None,
            source_device_id: None,
            source_seq: None,
        }];

        let result = SyncManager::apply_downloaded_changes(&conn, &changes, None).unwrap();
        assert_eq!(result.success_count, 1);

        let row_state: (i64, Option<String>) = conn
            .query_row(
                "SELECT COUNT(*), MAX(deleted_at) FROM test_records WHERE id = 'r1'",
                [],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .unwrap();
        assert_eq!(row_state.0, 1, "tombstone delete should keep row");
        assert!(row_state.1.is_some(), "deleted_at should be set");
    }

    #[test]
    fn test_apply_skips_incomplete_upsert_shadowed_by_batch_delete() {
        let conn = create_test_db_with_business_table();

        let changes = vec![
            SyncChangeWithData {
                table_name: "test_records".to_string(),
                record_id: "hard-deleted".to_string(),
                operation: ChangeOperation::Insert,
                data: None,
                changed_at: "2026-02-10T00:00:00Z".to_string(),
                change_log_id: Some(1),
                database_name: Some("vfs".to_string()),
                suppress_change_log: Some(true),
                source_device_id: None,
                source_seq: None,
            },
            SyncChangeWithData {
                table_name: "test_records".to_string(),
                record_id: "hard-deleted".to_string(),
                operation: ChangeOperation::Delete,
                data: None,
                changed_at: "2026-02-10T00:00:01Z".to_string(),
                change_log_id: Some(2),
                database_name: Some("vfs".to_string()),
                suppress_change_log: Some(true),
                source_device_id: None,
                source_seq: None,
            },
        ];

        let result = SyncManager::apply_downloaded_changes(&conn, &changes, None).unwrap();

        assert_eq!(result.success_count, 1, "DELETE should still be applied");
        assert_eq!(result.skipped_count, 1);
        assert_eq!(result.skipped_incomplete_count, 1);
        assert_eq!(result.failure_count, 0);
    }

    #[test]
    fn test_conflict_guard_skips_incomplete_upsert_shadowed_by_batch_delete() {
        let conn = create_test_db_with_business_table();

        let changes = vec![
            SyncChangeWithData {
                table_name: "test_records".to_string(),
                record_id: "hard-deleted".to_string(),
                operation: ChangeOperation::Insert,
                data: None,
                changed_at: "2026-02-10T00:00:00Z".to_string(),
                change_log_id: Some(1),
                database_name: Some("vfs".to_string()),
                suppress_change_log: Some(true),
                source_device_id: None,
                source_seq: None,
            },
            SyncChangeWithData {
                table_name: "test_records".to_string(),
                record_id: "hard-deleted".to_string(),
                operation: ChangeOperation::Delete,
                data: None,
                changed_at: "2026-02-10T00:00:01Z".to_string(),
                change_log_id: Some(2),
                database_name: Some("vfs".to_string()),
                suppress_change_log: Some(true),
                source_device_id: None,
                source_seq: None,
            },
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

        assert_eq!(result.success_count, 1, "DELETE should still be applied");
        assert_eq!(result.skipped_count, 1);
        assert_eq!(result.skipped_incomplete_count, 1);
        assert_eq!(result.failure_count, 0);
        assert_eq!(conflict_result.conflicts_saved, 0);
    }

    #[test]
    fn test_apply_downloaded_changes_rolls_back_on_fk_violation() {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(
            r#"
            PRAGMA foreign_keys = ON;
            CREATE TABLE parent_records (
                id TEXT PRIMARY KEY
            );
            CREATE TABLE child_records (
                id TEXT PRIMARY KEY,
                parent_id TEXT NOT NULL,
                FOREIGN KEY(parent_id) REFERENCES parent_records(id)
            );
            CREATE TABLE test_records (
                id TEXT PRIMARY KEY,
                content TEXT
            );
            "#,
        )
        .unwrap();

        let changes = vec![
            SyncChangeWithData {
                table_name: "test_records".to_string(),
                record_id: "safe-1".to_string(),
                operation: ChangeOperation::Insert,
                data: Some(serde_json::json!({
                    "id": "safe-1",
                    "content": "should rollback"
                })),
                changed_at: "2026-02-10T00:00:00Z".to_string(),
                change_log_id: None,
                database_name: None,
                suppress_change_log: None,
                source_device_id: None,
                source_seq: None,
            },
            SyncChangeWithData {
                table_name: "child_records".to_string(),
                record_id: "child-1".to_string(),
                operation: ChangeOperation::Insert,
                data: Some(serde_json::json!({
                    "id": "child-1",
                    "parent_id": "missing-parent"
                })),
                changed_at: "2026-02-10T00:00:01Z".to_string(),
                change_log_id: None,
                database_name: None,
                suppress_change_log: None,
                source_device_id: None,
                source_seq: None,
            },
        ];

        // 2026-06 语义更新：FK 违规属于非瞬态错误，违规的单条变更回滚到
        // SAVEPOINT 并进入检疫表，批次继续应用（不再整批失败）——
        // 与 regression_m10_poison_payload_is_quarantined_without_blocking_batch 一致。
        let result = SyncManager::apply_downloaded_changes(&conn, &changes, None)
            .expect("fk violation should be quarantined, not fail the batch");
        assert_eq!(result.success_count, 1, "safe record should be applied");
        assert_eq!(
            result.failure_count, 1,
            "violating record should be recorded as failure"
        );

        let test_records_count: i64 = conn
            .query_row("SELECT COUNT(*) FROM test_records", [], |row| row.get(0))
            .unwrap();
        let child_records_count: i64 = conn
            .query_row("SELECT COUNT(*) FROM child_records", [], |row| row.get(0))
            .unwrap();
        assert_eq!(test_records_count, 1, "safe record should be committed");
        assert_eq!(
            child_records_count, 0,
            "fk-violating record must not be committed"
        );

        // 违规记录应进入检疫表
        let quarantined: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM __sync_quarantine WHERE record_id = 'child-1'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(quarantined, 1, "violating record should be quarantined");
    }

    #[test]
    fn regression_m10_poison_payload_is_quarantined_without_blocking_batch() {
        let conn = create_test_db_with_business_table();

        let changes = vec![
            SyncChangeWithData {
                table_name: "test_records".to_string(),
                record_id: "bad-id".to_string(),
                operation: ChangeOperation::Insert,
                data: Some(json!({
                    "id": "different-id",
                    "content": "poison",
                    "updated_at": "2026-02-10T00:00:00Z"
                })),
                changed_at: "2026-02-10T00:00:00Z".to_string(),
                change_log_id: Some(1),
                database_name: None,
                suppress_change_log: Some(true),
                source_device_id: None,
                source_seq: None,
            },
            SyncChangeWithData {
                table_name: "test_records".to_string(),
                record_id: "good-id".to_string(),
                operation: ChangeOperation::Insert,
                data: Some(json!({
                    "id": "good-id",
                    "content": "applied",
                    "updated_at": "2026-02-10T00:00:01Z"
                })),
                changed_at: "2026-02-10T00:00:01Z".to_string(),
                change_log_id: Some(2),
                database_name: None,
                suppress_change_log: Some(true),
                source_device_id: None,
                source_seq: None,
            },
        ];

        let result = SyncManager::apply_downloaded_changes(&conn, &changes, None).unwrap();

        assert_eq!(result.success_count, 1);
        assert_eq!(result.failure_count, 1);
        assert_eq!(result.failures[0].record_id, "bad-id");

        let good_content: String = conn
            .query_row(
                "SELECT content FROM test_records WHERE id = 'good-id'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(good_content, "applied");

        let quarantine_count: i64 = conn
            .query_row("SELECT COUNT(*) FROM __sync_quarantine", [], |row| {
                row.get(0)
            })
            .unwrap();
        assert_eq!(quarantine_count, 1);
    }

    #[test]
    fn regression_m20_unregistered_table_payload_is_quarantined() {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(
            r#"
            CREATE TABLE evil_payloads (
                id TEXT PRIMARY KEY,
                content TEXT
            );
            "#,
        )
        .unwrap();

        let changes = vec![SyncChangeWithData {
            table_name: "evil_payloads".to_string(),
            record_id: "evil-1".to_string(),
            operation: ChangeOperation::Insert,
            data: Some(json!({
                "id": "evil-1",
                "content": "must not be written"
            })),
            changed_at: "2026-02-10T00:00:00Z".to_string(),
            change_log_id: None,
            database_name: Some("vfs".to_string()),
            suppress_change_log: None,
            source_device_id: None,
            source_seq: None,
        }];

        let result = SyncManager::apply_downloaded_changes(&conn, &changes, None).unwrap();

        assert_eq!(result.success_count, 0);
        assert_eq!(result.failure_count, 1);
        assert!(
            result.failures[0]
                .error
                .contains("禁止同步未注册为 RowSync 的表"),
            "unexpected failure: {:?}",
            result.failures[0]
        );

        let evil_count: i64 = conn
            .query_row("SELECT COUNT(*) FROM evil_payloads", [], |row| row.get(0))
            .unwrap();
        let quarantine_count: i64 = conn
            .query_row("SELECT COUNT(*) FROM __sync_quarantine", [], |row| {
                row.get(0)
            })
            .unwrap();

        assert_eq!(evil_count, 0);
        assert_eq!(quarantine_count, 1);
    }

    fn create_todo_constraint_test_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(
            r#"
            CREATE TABLE todo_lists (
                id TEXT PRIMARY KEY,
                title TEXT,
                updated_at TEXT
            );
            CREATE TABLE todo_items (
                id TEXT PRIMARY KEY,
                todo_list_id TEXT NOT NULL,
                title TEXT,
                status TEXT NOT NULL,
                priority TEXT NOT NULL,
                parent_id TEXT,
                updated_at TEXT,
                deleted_at TEXT
            );
            -- 与 V20260614 迁移保持一致：INSERT 要求 parent 行存在且同清单
            -- （软删除不影响）；UPDATE 仅在 parent 行物理存在时校验同清单。
            CREATE TRIGGER trg_todo_items_validate_insert
            BEFORE INSERT ON todo_items
            FOR EACH ROW
            BEGIN
                SELECT RAISE(ABORT, 'todo_items.parent_id must belong to the same list')
                WHERE NEW.parent_id IS NOT NULL
                  AND (
                    SELECT todo_list_id
                    FROM todo_items
                    WHERE id = NEW.parent_id
                  ) IS NOT NEW.todo_list_id;
            END;
            CREATE TRIGGER trg_todo_items_validate_update
            BEFORE UPDATE ON todo_items
            FOR EACH ROW
            BEGIN
                SELECT RAISE(ABORT, 'todo_items.parent_id must belong to the same list')
                WHERE NEW.parent_id IS NOT NULL
                  AND EXISTS (SELECT 1 FROM todo_items WHERE id = NEW.parent_id)
                  AND (
                    SELECT todo_list_id
                    FROM todo_items
                    WHERE id = NEW.parent_id
                  ) IS NOT NEW.todo_list_id;
            END;
            "#,
        )
        .unwrap();
        conn
    }

    fn todo_list_change() -> SyncChangeWithData {
        SyncChangeWithData {
            table_name: "todo_lists".to_string(),
            record_id: "list-1".to_string(),
            operation: ChangeOperation::Insert,
            data: Some(serde_json::json!({
                "id": "list-1",
                "title": "List",
                "updated_at": "2026-05-31T00:00:00Z"
            })),
            changed_at: "2026-05-31T00:00:00Z".to_string(),
            change_log_id: Some(1),
            database_name: Some("vfs".to_string()),
            suppress_change_log: Some(true),
            source_device_id: None,
            source_seq: None,
        }
    }

    fn todo_parent_change() -> SyncChangeWithData {
        SyncChangeWithData {
            table_name: "todo_items".to_string(),
            record_id: "todo-parent".to_string(),
            operation: ChangeOperation::Insert,
            data: Some(serde_json::json!({
                "id": "todo-parent",
                "todo_list_id": "list-1",
                "title": "Parent",
                "status": "pending",
                "priority": "high",
                "parent_id": null,
                "updated_at": "2026-05-31T00:00:00Z",
                "deleted_at": null
            })),
            changed_at: "2026-05-31T00:00:00Z".to_string(),
            change_log_id: Some(3),
            database_name: Some("vfs".to_string()),
            suppress_change_log: Some(true),
            source_device_id: None,
            source_seq: None,
        }
    }

    fn todo_child_change() -> SyncChangeWithData {
        SyncChangeWithData {
            table_name: "todo_items".to_string(),
            record_id: "todo-child".to_string(),
            operation: ChangeOperation::Insert,
            data: Some(serde_json::json!({
                "id": "todo-child",
                "todo_list_id": "list-1",
                "title": "Child",
                "status": "pending",
                "priority": "medium",
                "parent_id": "todo-parent",
                "updated_at": "2026-05-31T00:00:00Z",
                "deleted_at": null
            })),
            changed_at: "2026-05-31T00:00:00Z".to_string(),
            change_log_id: Some(2),
            database_name: Some("vfs".to_string()),
            suppress_change_log: Some(true),
            source_device_id: None,
            source_seq: None,
        }
    }

    fn assert_todo_parent_child_applied(conn: &Connection) {
        let child: (String, String) = conn
            .query_row(
                "SELECT parent_id, todo_list_id FROM todo_items WHERE id = 'todo-child'",
                [],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .unwrap();
        assert_eq!(child.0, "todo-parent");
        assert_eq!(child.1, "list-1");

        let parent_list: String = conn
            .query_row(
                "SELECT todo_list_id FROM todo_items WHERE id = 'todo-parent'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(parent_list, "list-1");
    }

    #[test]
    fn test_apply_downloaded_changes_orders_todo_parent_before_child() {
        let conn = create_todo_constraint_test_db();
        let changes = vec![
            todo_list_change(),
            todo_child_change(),
            todo_parent_change(),
        ];

        let result = SyncManager::apply_downloaded_changes(&conn, &changes, None).unwrap();

        assert_eq!(result.success_count, 3);
        assert_todo_parent_child_applied(&conn);
    }

    #[test]
    fn test_conflict_guard_orders_todo_parent_before_child() {
        let conn = create_todo_constraint_test_db();
        let changes = vec![
            todo_list_change(),
            todo_child_change(),
            todo_parent_change(),
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

        assert_eq!(result.success_count, 3);
        assert_eq!(conflict_result.conflicts_saved, 0);
        assert_todo_parent_child_applied(&conn);
    }

    fn create_vfs_blob_fk_test_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(
            r#"
            PRAGMA foreign_keys = ON;
            CREATE TABLE blobs (
                hash TEXT PRIMARY KEY,
                relative_path TEXT NOT NULL,
                size INTEGER NOT NULL,
                mime_type TEXT,
                ref_count INTEGER NOT NULL DEFAULT 0,
                created_at INTEGER NOT NULL
            );
            CREATE TABLE files (
                id TEXT PRIMARY KEY,
                blob_hash TEXT,
                sha256 TEXT NOT NULL UNIQUE,
                file_name TEXT NOT NULL,
                size INTEGER NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                FOREIGN KEY(blob_hash) REFERENCES blobs(hash)
            );
            "#,
        )
        .unwrap();
        conn
    }

    fn blob_metadata_change() -> SyncChangeWithData {
        SyncChangeWithData {
            table_name: "blobs".to_string(),
            record_id: "blob-hash-1".to_string(),
            operation: ChangeOperation::Insert,
            data: Some(serde_json::json!({
                "hash": "blob-hash-1",
                "relative_path": "bl/ob/blob-hash-1.md",
                "size": 12,
                "mime_type": "text/markdown",
                "ref_count": 1,
                "created_at": 1780225200000i64
            })),
            changed_at: "2026-05-31T00:00:01Z".to_string(),
            change_log_id: Some(2),
            database_name: Some("vfs".to_string()),
            suppress_change_log: Some(true),
            source_device_id: None,
            source_seq: None,
        }
    }

    fn blob_file_change() -> SyncChangeWithData {
        SyncChangeWithData {
            table_name: "files".to_string(),
            record_id: "file-1".to_string(),
            operation: ChangeOperation::Insert,
            data: Some(serde_json::json!({
                "id": "file-1",
                "blob_hash": "blob-hash-1",
                "sha256": "sha256-file-1",
                "file_name": "blob-backed.md",
                "size": 12,
                "created_at": "2026-05-31T00:00:00Z",
                "updated_at": "2026-05-31T00:00:00Z"
            })),
            changed_at: "2026-05-31T00:00:00Z".to_string(),
            change_log_id: Some(1),
            database_name: Some("vfs".to_string()),
            suppress_change_log: Some(true),
            source_device_id: None,
            source_seq: None,
        }
    }

    fn assert_blob_file_applied(conn: &Connection) {
        let row: (String, String) = conn
            .query_row(
                "SELECT files.blob_hash, blobs.relative_path
                 FROM files JOIN blobs ON blobs.hash = files.blob_hash
                 WHERE files.id = 'file-1'",
                [],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .unwrap();
        assert_eq!(row.0, "blob-hash-1");
        assert_eq!(row.1, "bl/ob/blob-hash-1.md");
    }

    #[test]
    fn test_quarantine_source_uses_v3_metadata() {
        let change = SyncChangeWithData {
            table_name: "notes".to_string(),
            record_id: "note-1".to_string(),
            operation: ChangeOperation::Update,
            data: None,
            changed_at: "2026-06-01T00:00:00Z".to_string(),
            change_log_id: Some(7),
            database_name: Some("vfs".to_string()),
            suppress_change_log: Some(true),
            source_device_id: Some("device-a".to_string()),
            source_seq: Some(42),
        };

        assert_eq!(
            SyncManager::source_device_for_quarantine(&change),
            "device-a"
        );
        assert_eq!(SyncManager::source_seq_for_quarantine(&change), 42);
    }

    #[test]
    fn test_apply_downloaded_changes_orders_blob_metadata_before_files() {
        let conn = create_vfs_blob_fk_test_db();
        let changes = vec![blob_file_change(), blob_metadata_change()];

        let result = SyncManager::apply_downloaded_changes(&conn, &changes, None).unwrap();

        assert_eq!(result.success_count, 2);
        assert_blob_file_applied(&conn);
    }

    #[test]
    fn regression_ref_count_is_recomputed_after_download_apply() {
        let conn = create_vfs_blob_fk_test_db();
        let mut blob = blob_metadata_change();
        if let Some(serde_json::Value::Object(obj)) = blob.data.as_mut() {
            obj.insert("ref_count".to_string(), serde_json::json!(99));
        }
        let changes = vec![blob_file_change(), blob];

        let result = SyncManager::apply_downloaded_changes(&conn, &changes, None).unwrap();

        assert_eq!(result.success_count, 2);
        let ref_count: i64 = conn
            .query_row(
                "SELECT ref_count FROM blobs WHERE hash='blob-hash-1'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(ref_count, 1);
    }

    /// ★ 2026-06-12（P1 防数据丢失）回归：仅存在于 JSON 中的 blob 引用
    /// （PDF 页图/压缩页图、试卷页图、题目图片）不得被重算清零。
    /// 修复前 recompute 只统计三个显式列，这些 blob 会被清零 → 启动清扫物理删除。
    #[test]
    fn regression_recompute_counts_json_embedded_blob_refs() {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(
            r#"
            CREATE TABLE blobs (
                hash TEXT PRIMARY KEY,
                ref_count INTEGER NOT NULL DEFAULT 0
            );
            CREATE TABLE files (
                id TEXT PRIMARY KEY,
                blob_hash TEXT,
                compressed_blob_hash TEXT,
